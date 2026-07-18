
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::SetBasicKey(int) */

void Widget::SetBasicKey(int param_1)

{
  undefined1 uVar1;
  code *pcVar2;
  int in_ECX;
  int iVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined *local_20;
  
  uVar4 = (uint)DAT_00002435;
  uVar6 = (uint)DAT_00002434;
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = &DAT_000022f1;
  iVar3 = uVar6 * 0x32 + uVar4 * 3000;
  *(undefined1 *)(iVar3 + 0x10b + (uint)*(byte *)(iVar3 + 0x109) * 2) =
       *(undefined1 *)(in_ECX + 0x800 + (param_1 & 0xffU));
  uVar1 = DAT_000000de;
  *(char *)(iVar3 + 0x109) = *(char *)(iVar3 + 0x109) + '\x01';
  *(undefined1 *)(iVar3 + 0x103) = uVar1;
  *(undefined1 *)(uVar6 + 0x40 + uVar4 * 0x3c) = 1;
  pcVar2 = ___imp___ZNK14QMessageLogger5debugEPKcz;
  (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_30,&DAT_000025b8,uVar4,uVar6);
  iVar3 = 0;
  do {
    local_30 = 2;
    local_2c = 0;
    local_28 = 0;
    local_24 = 0;
    local_20 = &DAT_000022f1;
    iVar5 = iVar3 + 1;
    (*pcVar2)(&local_30,&DAT_000025e2,iVar3,
              *(undefined1 *)((uint)DAT_00002434 * 0x32 + 0x100 + (uint)DAT_00002435 * 3000 + iVar3)
             );
    iVar3 = iVar5;
  } while (iVar5 != 0x32);
  return;
}

