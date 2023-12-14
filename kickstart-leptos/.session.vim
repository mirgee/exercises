let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Source/solidity-course/kickstart-leptos
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +31 Cargo.toml
badd +43 src/main.rs
badd +83 README.md
badd +318 ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos_config-0.5.4/src/lib.rs
badd +31 ~/Source/ethereum-course/kickstart/pages/index.js
badd +1 src/lib.rs
badd +1 src/ethereum.rs
badd +19 src/fileserv.rs
badd +1325 ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos_axum-0.5.4/src/lib.rs
badd +1 src/app.rs
badd +35 ethereum/compile.js
badd +39 src/error_template.rs
badd +12 src/ethereum/mod.rs
badd +590 ~/.local/state/nvim/lsp.log
badd +135 ~/.config/nvim/lua/mirgee/configs/lspconfig.lua
badd +28 ~/.config/nvim/lua/mirgee/plugins.lua
argglobal
%argdel
edit src/app.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
wincmd _ | wincmd |
vsplit
2wincmd h
wincmd w
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 90 + 68) / 136)
exe 'vert 2resize ' . ((&columns * 43 + 68) / 136)
exe 'vert 3resize ' . ((&columns * 1 + 68) / 136)
argglobal
balt Cargo.toml
setlocal fdm=indent
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal nofen
let s:l = 1 - ((0 * winheight(0) + 34) / 68)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("Cargo.toml", ":p")) | buffer Cargo.toml | else | edit Cargo.toml | endif
if &buftype ==# 'terminal'
  silent file Cargo.toml
endif
balt src/app.rs
setlocal fdm=indent
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal nofen
let s:l = 1 - ((0 * winheight(0) + 34) / 68)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("src/ethereum/mod.rs", ":p")) | buffer src/ethereum/mod.rs | else | edit src/ethereum/mod.rs | endif
if &buftype ==# 'terminal'
  silent file src/ethereum/mod.rs
endif
balt src/app.rs
setlocal fdm=indent
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal nofen
let s:l = 4 - ((3 * winheight(0) + 34) / 68)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 4
normal! 0
wincmd w
2wincmd w
exe 'vert 1resize ' . ((&columns * 90 + 68) / 136)
exe 'vert 2resize ' . ((&columns * 43 + 68) / 136)
exe 'vert 3resize ' . ((&columns * 1 + 68) / 136)
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
