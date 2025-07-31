_: {
  projectRootFile = "flake.nix";

  programs = {
    taplo.enable = true;
    rustfmt = {
      enable = true;
      edition = "2024";
    };
    nixfmt.enable = true;
  };
}
