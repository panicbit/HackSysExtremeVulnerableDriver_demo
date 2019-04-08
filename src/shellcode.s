USE32

pushad
xor eax,eax
mov eax,[fs:eax+0x124]
mov eax,[eax+0x50]
mov ecx,eax
mov edx,0x4
mov eax,[eax+0xb8]
sub eax,0xb8
cmp [eax+0xb4],edx
jnz 0x1a
mov edx,[eax+0xf8]
mov [ecx+0xf8],edx
popad
xor eax,eax
add esp,byte +0x24
pop ebp
ret 0x8