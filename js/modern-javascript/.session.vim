let SessionLoad = 1
if &cp | set nocp | endif
let s:cpo_save=&cpo
set cpo&vim
inoremap <expr> <Down> pumvisible() ? "\" : "\<Down>"
inoremap <expr> <S-Tab> pumvisible() ? "\" : "\<S-Tab>"
inoremap <expr> <Up> pumvisible() ? "\" : "\<Up>"
imap <Nul> <C-Space>
imap <C-G>S <Plug>ISurround
imap <C-G>s <Plug>Isurround
imap <C-S> <Plug>Isurround
imap <C-J> <Plug>IMAP_JumpForward
inoremap <silent> <Plug>IMAP_JumpBack =IMAP_Jumpfunc('b', 0)
inoremap <silent> <Plug>IMAP_JumpForward =IMAP_Jumpfunc('', 0)
nnoremap  :CtrlPBuffer
vmap <NL> <Plug>IMAP_JumpForward
nmap <NL> <Plug>IMAP_JumpForward
nnoremap  o
map  :NERDTreeToggle " Map shortcut to start
nnoremap <silent>  :CtrlP
nnoremap  :tabnew 
nnoremap e :call ToggleErrors()
vnoremap  `.``gvP``P
map [6;5~ :tabnext
map [5;5~ :tabprevious
xmap S <Plug>VSurround
map \nf :NERDTreeFind " Map shortcut to find file
map \d :YcmCompleter GetDoc
map \gt :tab split | YcmCompleter GoToDefinitionElseDeclaration
map \g :YcmCompleter GoToDefinitionElseDeclaration
map \bg :let &background = ( &background == "dark"? "light" : "dark" )
nnoremap \fs :Gstatus
nnoremap \fb :BCommits
nnoremap \fc :Commits
nnoremap \s :update
nnoremap \  :nohlsearch
nnoremap \lr :w:! lua5.3 %:p
nnoremap \pr :w:! python3 %:p
noremap <silent> \cu :silent s/^\V=escape(b:comment_leader,'\/')//e:nohlsearch
noremap <silent> \cc :silent s/^/=escape(b:comment_leader,'\/')/:nohlsearch
nmap cS <Plug>CSurround
nmap cs <Plug>Csurround
nmap ds <Plug>Dsurround
vmap gx <Plug>NetrwBrowseXVis
nmap gx <Plug>NetrwBrowseX
xmap gS <Plug>VgSurround
nnoremap gb :ls:b 
nmap ySS <Plug>YSsurround
nmap ySs <Plug>YSsurround
nmap yss <Plug>Yssurround
nmap yS <Plug>YSurround
nmap ys <Plug>Ysurround
nnoremap <SNR>80_: :=v:count ? v:count : ''
nnoremap <SNR>87_: :=v:count ? v:count : ''
vnoremap <silent> <Plug>NetrwBrowseXVis :call netrw#BrowseXVis()
nnoremap <silent> <Plug>NetrwBrowseX :call netrw#BrowseX(netrw#GX(),netrw#CheckIfRemote(netrw#GX()))
nnoremap <silent> <Plug>SurroundRepeat .
nnoremap <silent> <Plug>GitGutterPreviewHunk :GitGutterPreviewHunk
nnoremap <silent> <Plug>GitGutterUndoHunk :GitGutterUndoHunk
nnoremap <silent> <Plug>GitGutterStageHunk :GitGutterStageHunk
nnoremap <silent> <expr> <Plug>GitGutterPrevHunk &diff ? '[c' : ":\execute v:count1 . 'GitGutterPrevHunk'\"
nnoremap <silent> <expr> <Plug>GitGutterNextHunk &diff ? ']c' : ":\execute v:count1 . 'GitGutterNextHunk'\"
xnoremap <silent> <Plug>GitGutterTextObjectOuterVisual :call gitgutter#hunk#text_object(0)
xnoremap <silent> <Plug>GitGutterTextObjectInnerVisual :call gitgutter#hunk#text_object(1)
onoremap <silent> <Plug>GitGutterTextObjectOuterPending :call gitgutter#hunk#text_object(0)
onoremap <silent> <Plug>GitGutterTextObjectInnerPending :call gitgutter#hunk#text_object(1)
nnoremap <silent> <C-P> :CtrlP
vmap <C-J> <Plug>IMAP_JumpForward
nmap <C-J> <Plug>IMAP_JumpForward
vnoremap <silent> <Plug>IMAP_JumpBack `<i=IMAP_Jumpfunc('b', 0)
vnoremap <silent> <Plug>IMAP_JumpForward i=IMAP_Jumpfunc('', 0)
vnoremap <silent> <Plug>IMAP_DeleteAndJumpBack "_<Del>i=IMAP_Jumpfunc('b', 0)
vnoremap <silent> <Plug>IMAP_DeleteAndJumpForward "_<Del>i=IMAP_Jumpfunc('', 0)
nnoremap <silent> <Plug>IMAP_JumpBack i=IMAP_Jumpfunc('b', 0)
nnoremap <silent> <Plug>IMAP_JumpForward i=IMAP_Jumpfunc('', 0)
nnoremap <C-W>e :call ToggleErrors()
map <C-N> :NERDTreeToggle " Map shortcut to start
vnoremap <C-X> `.``gvP``P
nnoremap <C-B> :CtrlPBuffer
nnoremap <C-T> :tabnew 
nnoremap <S-CR> O
imap S <Plug>ISurround
imap s <Plug>Isurround
inoremap <expr> 	 pumvisible() ? "\" : "\	"
imap <NL> <Plug>IMAP_JumpForward
imap  <Plug>Isurround
abbr jj :w
abbr pd import pdb; pdb.set_trace()
let &cpo=s:cpo_save
unlet s:cpo_save
set autowrite
set backspace=indent,eol,start
set balloonexpr=SyntasticBalloonsExprNotifier()
set completefunc=youcompleteme#CompleteFunc
set completeopt=preview,menuone
set cpoptions=aAceFsB
set expandtab
set fileencodings=ucs-bom,utf-8,default,latin1
set guioptions=aegirt
set helplang=en
set hlsearch
set ignorecase
set laststatus=2
set mouse=a
set printoptions=paper:a4
set ruler
set runtimepath=~/.vim,~/.vim/bundle/Vundle.vim,~/.vim/bundle/YouCompleteMe,~/.vim/bundle/SimpylFold,~/.vim/bundle/syntastic,~/.vim/bundle/vim-isort,~/.vim/bundle/nerdtree,~/.vim/bundle/ctrlp.vim,~/.vim/bundle/vim-ripgrep,~/.vim/bundle/vim-fugitive,~/.vim/bundle/vim-gitgutter,~/.vim/bundle/lightline.vim,~/.vim/bundle/nerdtree-git-plugin,~/.vim/bundle/vim-nerdtree-syntax-highlight,~/.vim/bundle/vim-surround,~/.vim/bundle/indentpython.vim,~/.vim/bundle/gruvbox,/var/lib/vim/addons,/etc/vim,/usr/share/vim/vimfiles,/usr/share/vim/vim81,/usr/share/vim/vimfiles/after,/etc/vim/after,/var/lib/vim/addons/after,~/.vim/after,~/.vim/bundle/Vundle.vim,~/.vim/bundle/Vundle.vim/after,~/.vim/bundle/YouCompleteMe/after,~/.vim/bundle/SimpylFold/after,~/.vim/bundle/syntastic/after,~/.vim/bundle/vim-isort/after,~/.vim/bundle/nerdtree/after,~/.vim/bundle/ctrlp.vim/after,~/.vim/bundle/vim-ripgrep/after,~/.vim/bundle/vim-fugitive/after,~/.vim/bundle/vim-gitgutter/after,~/.vim/bundle/lightline.vim/after,~/.vim/bundle/nerdtree-git-plugin/after,~/.vim/bundle/vim-nerdtree-syntax-highlight/after,~/.vim/bundle/vim-surround/after,~/.vim/bundle/indentpython.vim/after,~/.vim/bundle/gruvbox/after
set scrolloff=3
set shiftwidth=2
set shortmess=filnxtToOSc
set showcmd
set softtabstop=2
set splitbelow
set splitright
set statusline=%#warningmsg#%{SyntasticStatuslineFlag()}%*
set suffixes=.bak,~,.swp,.o,.info,.aux,.log,.dvi,.bbl,.blg,.brf,.cb,.ind,.idx,.ilg,.inx,.out,.toc
set noswapfile
set tabline=%!lightline#tabline()
set tabstop=2
set tags=tags
set viewoptions=cursor,folds,slash,unix
set wildmenu
set nowritebackup
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Source/tutorials/js/modern-javascript
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
argglobal
%argdel
$argadd 1.html
edit calculator.html
set splitbelow splitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
vnoremap <buffer> 	 :py3 isort_visual()
let s:cpo_save=&cpo
set cpo&vim
nmap <buffer> [c <Plug>GitGutterPrevHunk
nmap <buffer> \hp <Plug>GitGutterPreviewHunk
nmap <buffer> \hu <Plug>GitGutterUndoHunk
nmap <buffer> \hs <Plug>GitGutterStageHunk
nmap <buffer> ]c <Plug>GitGutterNextHunk
xmap <buffer> ac <Plug>GitGutterTextObjectOuterVisual
omap <buffer> ac <Plug>GitGutterTextObjectOuterPending
xmap <buffer> ic <Plug>GitGutterTextObjectInnerVisual
omap <buffer> ic <Plug>GitGutterTextObjectInnerPending
vnoremap <buffer> <C-I> :py3 isort_visual()
let &cpo=s:cpo_save
unlet s:cpo_save
setlocal keymap=
setlocal noarabic
setlocal autoindent
setlocal backupcopy=
setlocal balloonexpr=
setlocal nobinary
setlocal nobreakindent
setlocal breakindentopt=
setlocal bufhidden=
setlocal buflisted
setlocal buftype=
setlocal nocindent
setlocal cinkeys=0{,0},0),0],:,0#,!^F,o,O,e
setlocal cinoptions=
setlocal cinwords=if,else,while,do,for,switch
set colorcolumn=0
setlocal colorcolumn=0
setlocal comments=s:<!--,m:\ \ \ \ ,e:-->
setlocal commentstring=<!--%s-->
setlocal complete=.,w,b,u,t,i
setlocal concealcursor=
setlocal conceallevel=0
setlocal completefunc=youcompleteme#CompleteFunc
setlocal nocopyindent
setlocal cryptmethod=
setlocal nocursorbind
setlocal nocursorcolumn
set cursorline
setlocal cursorline
setlocal cursorlineopt=both
setlocal define=
setlocal dictionary=
setlocal nodiff
setlocal equalprg=
setlocal errorformat=
setlocal expandtab
if &filetype != 'html'
setlocal filetype=html
endif
setlocal fixendofline
setlocal foldcolumn=0
setlocal foldenable
setlocal foldexpr=0
setlocal foldignore=#
set foldlevel=99
setlocal foldlevel=99
setlocal foldmarker={{{,}}}
set foldmethod=indent
setlocal foldmethod=indent
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldtext=foldtext()
setlocal formatexpr=
setlocal formatoptions=tcq
setlocal formatlistpat=^\\s*\\d\\+[\\]:.)}\\t\ ]\\s*
setlocal formatprg=
setlocal grepprg=
setlocal iminsert=0
setlocal imsearch=-1
setlocal include=
setlocal includeexpr=
setlocal indentexpr=HtmlIndent()
setlocal indentkeys=o,O,<Return>,<>>,{,},!^F
setlocal noinfercase
setlocal iskeyword=@,48-57,_,192-255
setlocal keywordprg=
setlocal nolinebreak
setlocal nolisp
setlocal lispwords=
setlocal nolist
setlocal makeencoding=
setlocal makeprg=
setlocal matchpairs=(:),{:},[:],<:>
setlocal modeline
setlocal modifiable
setlocal nrformats=bin,octal,hex
set number
setlocal number
setlocal numberwidth=4
setlocal omnifunc=htmlcomplete#CompleteTags
setlocal path=
setlocal nopreserveindent
setlocal nopreviewwindow
setlocal quoteescape=\\
setlocal noreadonly
setlocal norelativenumber
setlocal norightleft
setlocal rightleftcmd=search
setlocal noscrollbind
setlocal scrolloff=-1
setlocal shiftwidth=2
setlocal noshortname
setlocal showbreak=
setlocal sidescrolloff=-1
setlocal signcolumn=auto
setlocal nosmartindent
setlocal softtabstop=2
setlocal nospell
setlocal spellcapcheck=[.?!]\\_[\\])'\"\	\ ]\\+
setlocal spellfile=
setlocal spelllang=en
setlocal statusline=%{lightline#link()}%#LightlineLeft_active_0#%(\ %{lightline#mode()}\ %)%{(&paste)?\"|\":\"\"}%(\ %{&paste?\"PASTE\":\"\"}\ %)%#LightlineLeft_active_0_1#%#LightlineLeft_active_1#%(\ %{LightlineFugitive()}\ %)%{LightlineFugitive()!=#\"\"&&(1||(&modified||!&modifiable))?\"|\":\"\"}%(\ %f\ %)%{(&modified||!&modifiable)?\"|\":\"\"}%(\ %M\ %)%#LightlineLeft_active_1_2#%#LightlineMiddle_active#%=%#LightlineRight_active_2_3#%#LightlineRight_active_2#%(\ %{&ff}\ %)%{1||1||1?\"|\":\"\"}%(\ %{&fenc!=#\"\"?&fenc:&enc}\ %)%{1||1?\"|\":\"\"}%(\ %{&ft!=#\"\"?&ft:\"no\ ft\"}\ %)%{1?\"|\":\"\"}%(\ %B\ %)%#LightlineRight_active_1_2#%#LightlineRight_active_1#%(\ %3p%%\ %)%#LightlineRight_active_0_1#%#LightlineRight_active_0#%(\ %3l:%-2v\ %)
setlocal suffixesadd=
setlocal noswapfile
setlocal synmaxcol=3000
if &syntax != 'html'
setlocal syntax=html
endif
setlocal tabstop=2
setlocal tagcase=
setlocal tagfunc=
setlocal tags=
setlocal termwinkey=
setlocal termwinscroll=10000
setlocal termwinsize=
setlocal textwidth=0
setlocal thesaurus=
setlocal noundofile
setlocal undolevels=-123456
setlocal varsofttabstop=
setlocal vartabstop=
setlocal wincolor=
setlocal nowinfixheight
setlocal nowinfixwidth
setlocal wrap
setlocal wrapmargin=0
4
normal! zo
let s:l = 17 - ((16 * winheight(0) + 23) / 46)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
17
normal! 011|
tabnext 1
badd +5 calculator.html
badd +8 1.html
badd +8 pow.js
badd +4 backticks.js
badd +3 assign.js
badd +1 alert.js
badd +15 conversions.js
badd +7 namofjs.js
badd +6 nameofjs.js
badd +11 showthesign.js
badd +7 terciary.js
badd +3 boolofundef.js
badd +7 repeatuntil.js
badd +10 primenumbers.js
badd +0 filterRange.html
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=filnxtToOSc
set winminheight=1 winminwidth=1
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
