{ config, pkgs, lib, ... }:

let
  cfg = config.services.system-notifier;
  dbusName = "me.section.Notifier";

  notifier-pkg = pkgs.rustPlatform.buildRustPackage {
    meta = with lib; {
      description = "Small utility to allow systemd units to send notifications to graphically logged in users via dbus.";
      longDescription = ''
    GNU Hello is a program that prints "Hello, world!" when you run it.
    It is fully customizable.
  '';
      homepage = "https://www.github.com/shift/dots-notifier/";
      license = licenses.gpl3Plus;
      maintainers = with maintainers; [ shift ];
      platforms = platforms.all;
      mainProgram = "dots-notifier";
    };
    pname = "dots-notifier";
    version = "0.1.0";
    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;

    nativeBuildInputs = with pkgs; [ pkg-config ];
    buildInputs = with pkgs; [ dbus systemd ];
  };

  dbusServiceFile = pkgs.writeText "dbus-notifier.service" ''
    [D-BUS Service]
    Name=${dbusName}
    Exec=${lib.getExe notifier-pkg} server
    User=root
  '';

  dbusPolicyFile = pkgs.writeText "dbus-notifier.conf" ''
    <!DOCTYPE busconfig PUBLIC "-//freedesktop//DTD D-BUS Bus Configuration 1.0//EN"
     "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
    <busconfig>
      <policy user="root">
        <allow own="${dbusName}"/>
      </policy>

      <policy context="default">
        <allow own="${dbusName}"/>
        <allow send_destination="${dbusName}"/>
      </policy>

      <policy group="wheel">
        <allow send_destination="${dbusName}"
           send_interface="${dbusName}"
           send_member="SendToAll"/>
      </policy>

      <policy group="broadcast">
        <allow send_destination="${dbusName}"
           send_interface="${dbusName}"
           send_member="SendToAll"/>
      </policy>

      <policy context="default">
        <allow send_destination="${dbusName}"
           send_interface="${dbusName}"
           send_member="SendToAll"/>
      </policy>
    </busconfig>
  '';

in
{
  options.services.system-notifier = {
    enable = lib.mkEnableOption "the dots system-wide notification service";

    group = lib.mkOption {
      type = lib.types.str;
      default = "wheel";
      description = "The group whose members are allowed to send system-wide notifications.";
    };
  };

  config = lib.mkIf cfg.enable {
    users.groups.${cfg.group} = {};
    environment.systemPackages = [ notifier-pkg ];

    services.dbus.packages = [
      (pkgs.runCommand "dbus-notifier-config" {} ''
        mkdir -p $out/share/dbus-1/system-services
        cp ${dbusServiceFile} $out/share/dbus-1/system-services/${dbusName}.service
        
        mkdir -p $out/share/dbus-1/system.d
        cp ${dbusPolicyFile} $out/share/dbus-1/system.d/${dbusName}.conf
      '')
    ];

    security.polkit.extraConfig = ''
      polkit.addRule(function(action, subject) {
        if (action.id == "${dbusName}.send_to_all") {
          if (subject.isInGroup("${cfg.group}")) {
            return polkit.Result.YES;
          } else {
            return polkit.Result.NO;
          }
        }
      });
    '';
  };
}
