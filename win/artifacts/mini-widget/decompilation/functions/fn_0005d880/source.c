
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::timeoutSlot() */

void Widget::timeoutSlot(void)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  int in_ECX;
  int iVar3;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined *local_10;
  
  if (_DAT_00000024 != (undefined4 *)0x0) {
    _DAT_00000024 = (undefined4 *)_hid_enumerate(_DAT_000000fa,_DAT_000000f8);
    if (_DAT_00000024 == (undefined4 *)0x0) {
      _hid_close(*(undefined4 *)(in_ECX + 0x968));
      _hid_exit();
      DAT_0000242c = 0;
      (*___imp___ZN7QWidget4hideEv)();
    }
    else {
      _hid_free_enumeration(_DAT_00000024);
      Display_Dev_Connect();
      if (DAT_0000242c == '\0') {
        Read_KeyBoard_KeyNum();
      }
      DAT_0000242c = 1;
    }
    if ((DAT_00002428 == DAT_000000db) && (DAT_000000dc == DAT_00002429)) {
      return;
    }
    DAT_000000db = DAT_00002428;
    DAT_000000dd = DAT_0000242a;
    DAT_000000dc = DAT_00002429;
    Identify_KeyBoard_style();
    (*___imp___ZN7QWidget4hideEv)();
    return;
  }
  iVar3 = 0;
  while (_DAT_00000024 =
              (undefined4 *)_hid_enumerate(_DAT_000000fa,*(undefined2 *)(iVar3 * 2 + 0xe8)),
        _DAT_00000024 == (undefined4 *)0x0) {
    iVar3 = iVar3 + 1;
    if (iVar3 == 7) {
      iVar3 = 0;
      goto LAB_0005d966;
    }
    _DAT_00000024 = (undefined4 *)0x0;
  }
  _DAT_000000f8 = *(undefined2 *)(iVar3 * 2 + 0xe8);
  goto LAB_0005d9a1;
  while (iVar3 = iVar3 + 1, iVar3 != 8) {
LAB_0005d966:
    _DAT_00000024 = (undefined4 *)0x0;
    _DAT_00000024 = (undefined4 *)_hid_enumerate(&DAT_0000514c,*(undefined2 *)(iVar3 * 2 + 0xe8));
    if (_DAT_00000024 != (undefined4 *)0x0) {
      _DAT_000000fa = 0x514c;
      _DAT_000000f8 = *(undefined2 *)(iVar3 * 2 + 0xe8);
      break;
    }
  }
LAB_0005d9a1:
  _DAT_00000024 = (undefined4 *)_hid_enumerate(_DAT_000000fa,_DAT_000000f8);
  if (_DAT_00000024 == (undefined4 *)0x0) {
    Display_Dev_Disconnect();
    DAT_0000242c = 0;
    return;
  }
  iVar3 = _DAT_00000024[7];
  do {
    if (iVar3 == 0) {
      local_20 = 2;
      local_1c = 0;
      local_18 = 0;
      local_14 = 0;
      local_10 = &DAT_000022f1;
      (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_20,&DAT_00002d50,*_DAT_00000024);
      puVar1 = _DAT_00000024;
LAB_0005da5d:
      uVar2 = _hid_open_path(*puVar1);
      *(undefined4 *)(in_ECX + 0x968) = uVar2;
      _hid_free_enumeration(_DAT_00000024);
      return;
    }
    puVar1 = (undefined4 *)_DAT_00000024[8];
    if (puVar1 == (undefined4 *)0x0) {
      _DAT_00000024 = (undefined4 *)0x0;
      goto LAB_0005da5d;
    }
    iVar3 = puVar1[7];
    _DAT_00000024 = puVar1;
  } while( true );
}

