
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::SetFunKey(int) */

void Widget::SetFunKey(int param_1)

{
  byte *pbVar1;
  byte bVar2;
  code *pcVar3;
  uint uVar4;
  int in_ECX;
  uint uVar5;
  int iVar6;
  int iVar7;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined *local_20;
  
  uVar4 = (uint)DAT_00002435;
  uVar5 = (uint)DAT_00002434;
  iVar7 = uVar5 * 0x32 + 0x100 + uVar4 * 3000;
  iVar6 = (uint)*(byte *)(iVar7 + 9) * 2 + 10;
  bVar2 = *(byte *)(iVar7 + iVar6);
  if (((byte)param_1 < 0x14) || (*(byte *)(in_ECX + 0x45) < (byte)param_1)) {
    *(byte *)(uVar5 * 0x32 + 0x100 + uVar4 * 3000 + iVar6) =
         bVar2 | *(byte *)(in_ECX + 0x800 + (param_1 & 0xffU));
  }
  else {
    pbVar1 = (byte *)(iVar7 + 1 + iVar6);
    *(byte *)(iVar7 + iVar6) = bVar2 | 2;
    *pbVar1 = *pbVar1 | *(byte *)(in_ECX + 0x800 + (param_1 & 0xffU));
    *(char *)(iVar7 + 9) = *(char *)(iVar7 + 9) + '\x01';
  }
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = &DAT_000022f1;
  *(undefined1 *)(uVar5 * 0x32 + 0x103 + uVar4 * 3000) = DAT_000000de;
  pcVar3 = ___imp___ZNK14QMessageLogger5debugEPKcz;
  *(undefined1 *)(uVar5 + 0x40 + uVar4 * 0x3c) = 1;
  (*pcVar3)(&local_30,&DAT_000025b8,uVar4,uVar5);
  iVar6 = 0;
  do {
    local_30 = 2;
    local_2c = 0;
    local_28 = 0;
    local_24 = 0;
    local_20 = &DAT_000022f1;
    iVar7 = iVar6 + 1;
    (*pcVar3)(&local_30,&DAT_000025e2,iVar6,
              *(undefined1 *)((uint)DAT_00002434 * 0x32 + 0x100 + (uint)DAT_00002435 * 3000 + iVar6)
             );
    iVar6 = iVar7;
  } while (iVar7 != 0x32);
  return;
}

