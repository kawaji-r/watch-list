## DevContainer
```
devcontainer up --additional-features='{"ghcr.io/duduribeiro/devcontainer-features/neovim:1": {}}' --mount type=bind,source=~/.config/nvim,target=/nvim-config/nvim --workspace-folder .
devcontainer exec --remote-env XDG_CONFIG_HOME=/nvim-config --workspace-folder . nvim
(cd docker; docker compose down)
```
