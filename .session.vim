let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Source/ethereum-course
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +8 ballot/package.json
badd +36 ballot/contracts/Lottery.sol
badd +1 inbox/contracts/Inbox.sol
badd +23 ballot/compile.js
badd +29 ballot/deploy.js
badd +105 ballot/test/Lottery.test.js
badd +13 lottery-react/package.json
badd +1 lottery-react/public/index.html
badd +16 inbox/package.json
badd +1 lottery-react/src/index.js
badd +13 lottery-react/src/App.js
badd +1 lottery-react/src/web3.js
badd +74 lottery-react/src/lottery.js
badd +26 .gitignore
badd +1 campaign/contracts/Campaign.sol
badd +10 kickstart/package.json
badd +29 kickstart/ethereum/deploy.js
badd +11 kickstart/ethereum/compile.js
badd +94 kickstart/ethereum/contracts/Campaign.sol
badd +1 inbox/compile.js
badd +55 kickstart/test/Campaign.test.js
badd +7 inbox/test/Inbox.test.js
badd +1 kickstart/ethereum/build/Campaign.json
badd +1 kickstart/ethereum/build/CampaignFactory.json
badd +1 inbox/deploy.js
badd +1 kickstart/ADDRESS.md
badd +12 kickstart/pages/index.js
badd +17 kickstart/ethereum/web3.js
badd +6 kickstart/ethereum/factory.js
badd +1 lottery-react/src/setupTests.js
badd +3 kickstart/components/Layout.js
badd +17 kickstart/components/Header.js
badd +33 kickstart/pages/campaigns/new.js
badd +24 kickstart/pages/campaigns/\[address].js
badd +10 kickstart/ethereum/campaign.js
badd +28 kickstart/components/ContributeForm.js
badd +16 kickstart/pages/campaigns/\[address]/requests.js
badd +31 kickstart/pages/campaigns/\[address]/requests/new.js
argglobal
%argdel
edit kickstart/ethereum/contracts/Campaign.sol
argglobal
balt kickstart/pages/campaigns/\[address]/requests/new.js
setlocal fdm=indent
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal nofen
let s:l = 93 - ((28 * winheight(0) + 35) / 71)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 93
normal! 0
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
