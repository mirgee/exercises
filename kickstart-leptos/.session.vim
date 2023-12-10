let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Source/ethereum-course/kickstart-leptos
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
badd +1 src/main.rs
badd +1 README.md
badd +318 ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos_config-0.5.4/src/lib.rs
badd +31 ~/Source/ethereum-course/kickstart/pages/index.js
badd +5 src/lib.rs
badd +1 src/ethereum.rs
badd +1 src/fileserv.rs
badd +1325 ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos_axum-0.5.4/src/lib.rs
badd +49 src/app.rs
badd +35 ethereum/compile.js
badd +1 src/error_template.rs
badd +12 src/ethereum/mod.rs
argglobal
%argdel
edit src/ethereum/mod.rs
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
let s:l = 10 - ((9 * winheight(0) + 35) / 71)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 10
normal! 036|
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
