{ config, pkgs, lib, ... }:

let
  cfg = config.services.system-notifier;
  dbusName = "me.section.Notifier";

  notifier-pkg = pkgs.rustPlatform.buildRustPackage {
    pname = "dots-notify";
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

    dbus.packages = [
      (pkgs.runCommand "dbus-service-dir" {} ''
        mkdir -p $out/share/dbus-1/system-services
        cp ${dbusServiceFile} $out/share/dbus-1/system-services/${dbusName}.service
      '')
    ];

    security.polkit.extraRules = ''
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
    ];
  };
}
