# ghostty-lsp

LSP for [Ghostty](https://github.com/mitchellh/ghostty) providing hover definitions for each parameter in Ghostty's configuration.

## Installation

Install using the Rust package manager.

`cargo build --release`

Make sure the created binary is in your `$PATH`.

### Neovim

Add the following LSP configuration to have the LSP only activate on the Ghostty config (this assumes you use default location for Ghostty config - if not update the normalized location to your needs).

```lua
function setup_ghostty_lsp()
    -- Only activate ghostty-lsp when editing the ghostty config
    if vim.fn.expand("%:p") == vim.fs.normalize("~/.config/ghostty/config") then
        vim.lsp.start({
            name = "ghostty-lsp",
            cmd = { "ghostty-lsp" },
            root_dir = vim.fs.normalize("~/.config/ghostty")
        })
    end
end

vim.api.nvim_create_autocmd("BufRead", { pattern = "*", callback = setup_ghostty_lsp })
```


