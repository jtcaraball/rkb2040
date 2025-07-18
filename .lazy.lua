vim.lsp.config('rust_analyzer', {
	settings = {
		['rust-analyzer'] = {
			check = {
				command = "clippy",
			},
			cargo = {
				target = "thumbv6m-none-eabi",
				allTargets = false,
			}
		}
	}
})

return {}
