
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::SetMulKey(int) */

void Widget::SetMulKey(int param_1)

{
  undefined1 *puVar1;
  undefined1 uVar2;
  code *pcVar3;
  int in_ECX;
  uint uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined *local_20;
  
  uVar4 = (uint)DAT_00002434;
  uVar7 = (uint)DAT_00002435;
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = &DAT_000022f1;
  iVar5 = uVar4 * 0x32 + 0x100 + uVar7 * 3000;
  puVar1 = (undefined1 *)(iVar5 + 10 + (uint)*(byte *)(iVar5 + 9) * 2);
  *puVar1 = *(undefined1 *)(in_ECX + 0x800 + (param_1 & 0xffU));
  puVar1[1] = *(undefined1 *)(in_ECX + 0x801 + (param_1 & 0xffU));
  uVar2 = DAT_000000de;
  *(char *)(iVar5 + 9) = *(char *)(iVar5 + 9) + '\x02';
  pcVar3 = ___imp___ZNK14QMessageLogger5debugEPKcz;
  *(undefined1 *)(iVar5 + 3) = uVar2;
  *(undefined1 *)(uVar4 + 0x40 + uVar7 * 0x3c) = 1;
  (*pcVar3)(&local_30,&DAT_000025b8,uVar7,uVar4);
  iVar5 = 0;
  do {
    local_30 = 2;
    local_2c = 0;
    local_28 = 0;
    local_24 = 0;
    local_20 = &DAT_000022f1;
    iVar6 = iVar5 + 1;
    (*pcVar3)(&local_30,&DAT_000025e2,iVar5,
              *(undefined1 *)((uint)DAT_00002434 * 0x32 + 0x100 + (uint)DAT_00002435 * 3000 + iVar5)
             );
    iVar5 = iVar6;
  } while (iVar6 != 0x32);
  return;
}

