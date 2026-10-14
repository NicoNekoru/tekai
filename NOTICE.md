# Tekai licensing

Copyright 2026 Tekai contributors.

Tekai's CLI and the `tekai-engine` and `tekai-pdftex` Rust crates are free
software. You may redistribute and modify them under the GNU General Public
License as published by the Free Software Foundation, version 2 or any later
version. They are distributed without any warranty, including the implied
warranties of merchantability or fitness for a particular purpose. The full
license text is in [LICENSE](LICENSE).

The exact engine derives from pdfTeX. Translating and modifying its code does
not replace the upstream grant with MIT. The applicable upstream copyright,
license and modification notices are in
[`crates/tekai-engine/NOTICE.md`](crates/tekai-engine/NOTICE.md). The current Rust
source is the maintained implementation, rather than an unmodified upstream
pdfTeX distribution.

## Separately licensed material

- [LICENSE-MIT](LICENSE-MIT) preserves the previous grant for independently
  authored material that its copyright holders validly licensed under MIT.
  Those permissions remain available. This is not an MIT grant for imported
  pdfTeX code or for the combined GPL distribution.
- The [VS Code](editors/vscode/LICENSE) and [Neovim](editors/nvim/LICENSE)
  clients remain MIT. They invoke the CLI as a separate process. VS Code's
  third-party viewer assets retain their own notices.
- Vendored Tectonic adapters retain their MIT grants and
  [upstream notice](vendor/LICENSE.tectonic). The pinned `fdeflate` decoder
  retains its [source notice](vendor/fdeflate/SOURCE.md) and MIT/Apache-2.0
  grants. Its MIT option is available for GPL-compatible distribution.
- Cargo dependencies retain their own grants. `Cargo.lock` identifies their
  exact versions and registry checksums. In particular, `option-ext` retains
  MPL-2.0 notices and its permitted GPL secondary licensing for this larger
  work. A future binary package must include dependency notices and provide
  the corresponding source required by the applicable licenses.
- Bundled TeX packages and fonts retain the licenses and notices inside the
  [runtime archives](runtime/README.md). Their licenses include LPPL, OFL,
  Apache-2.0 and other terms. The program's GPL grant does not relicense those
  independent files. The archived `README.Tekai` mentions the historical root
  MIT license only to exclude bundled package data from that license. Use this
  notice for the program's current licensing.
- The two arXiv paper fixtures retain their authors' Creative Commons grants
  and the separate notices of included TeX styles. Their attribution, exact
  source versions and licensing are recorded in [examples/NOTICE](examples/NOTICE).
  The program's GPL grant does not relicense these documents or third-party
  styles.

The repository's tagged source archives include the maintained Rust source,
build scripts, vendored code, runtime data and the notices above. They are not
represented as a complete dependency-source bundle for a binary release.
Generated documents and user inputs retain their own copyrights and licenses.
