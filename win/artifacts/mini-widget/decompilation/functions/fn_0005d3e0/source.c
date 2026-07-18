
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::Read_KeyBoard_KeyNum() */

void Widget::Read_KeyBoard_KeyNum(void)

{
  code *pcVar1;
  int iVar2;
  undefined4 uVar3;
  int in_ECX;
  uint uVar4;
  undefined4 *puVar5;
  undefined4 local_a4;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined4 local_98;
  undefined *local_94;
  undefined4 local_8e [16];
  undefined1 local_4d [2];
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 local_49;
  undefined4 local_10;
  
  _local_4d = 0;
  local_10 = 0;
  puVar5 = (undefined4 *)((int)local_4d + 1U);
  for (uVar4 = (uint)(&stack0xfffffff4 + -((int)local_4d + 1U)) >> 2; uVar4 != 0; uVar4 = uVar4 - 1)
  {
    *puVar5 = 0;
    puVar5 = puVar5 + 1;
  }
  local_8e[0] = 0xfbfbfb03;
  iVar2 = _hid_write(*(undefined4 *)(in_ECX + 0x968),local_8e,0x41);
  if (-1 < iVar2) {
    iVar2 = _hid_read_timeout(*(undefined4 *)(in_ECX + 0x968),local_4d,0x40,10);
    pcVar1 = ___imp___ZNK14QMessageLogger5debugEPKcz;
    if (-1 < iVar2) {
      local_a4 = 2;
      local_a0 = 0;
      local_9c = 0;
      local_98 = 0;
      local_94 = &DAT_000022f1;
      if (iVar2 != 0) {
        (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_a4,&DAT_00002cbe,&DAT_00002cd5);
        local_a4 = 2;
        local_a0 = 0;
        local_9c = 0;
        local_98 = 0;
        local_94 = &DAT_000022f1;
        (*pcVar1)(&local_a4,&DAT_00002cdd,_local_4d >> 0x10 & 0xff,_local_4d >> 0x18);
        DAT_000000db = uStack_4b;
        DAT_00002428 = uStack_4b;
        DAT_000000dc = uStack_4a;
        DAT_000000dd = local_49;
        DAT_00002429 = uStack_4a;
        DAT_0000242a = local_49;
        Identify_KeyBoard_style();
        (*___imp___ZN7QWidget4hideEv)();
        (*___imp___ZN7QWidget10setEnabledEb)(1);
        return;
      }
      (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_a4,&DAT_00002cbe,&DAT_00002cb6);
      (*___imp___ZN7QWidget4showEv)();
      (*___imp___ZN7QWidget10setEnabledEb)(0);
      return;
    }
  }
  local_a4 = 2;
  local_a0 = 0;
  local_9c = 0;
  local_98 = 0;
  local_94 = &DAT_000022f1;
  uVar3 = _hid_error(*(undefined4 *)(in_ECX + 0x968));
  (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_a4,&DAT_000022f9,uVar3);
  return;
}

