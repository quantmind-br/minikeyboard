
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::SetMousePage(int) */

void Widget::SetMousePage(int param_1)

{
  undefined1 uVar1;
  code *pcVar2;
  byte bVar3;
  int in_ECX;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined *local_10;
  
  pcVar2 = ___imp___ZNK14QMessageLogger5debugEPKcz;
  uVar1 = DAT_000000de;
  uVar4 = (uint)DAT_00002435;
  uVar5 = (uint)DAT_00002434;
  bVar3 = *(char *)(in_ECX + 0x800 + (param_1 & 0xffU)) + 0x9f;
  if (bVar3 < 0xb) {
                    /* WARNING: Could not emulate address calculation at 0x0000bfee */
                    /* WARNING: Treating indirect jump as call */
    (**(code **)(&DAT_00002614 + (uint)bVar3 * 4))();
    return;
  }
  local_20 = 2;
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  local_10 = &DAT_000022f1;
  iVar6 = uVar5 * 0x32 + 0x100 + uVar4 * 3000;
  *(undefined1 *)(iVar6 + 9) = 1;
  *(undefined1 *)(iVar6 + 3) = uVar1;
  *(undefined1 *)(uVar5 + 0x40 + uVar4 * 0x3c) = 1;
  (*pcVar2)(&local_20,&DAT_000025b8,uVar4,uVar5);
  iVar6 = 0;
  do {
    local_20 = 2;
    local_1c = 0;
    local_18 = 0;
    local_14 = 0;
    local_10 = &DAT_000022f1;
    iVar7 = iVar6 + 1;
    (*pcVar2)(&local_20,&DAT_000025e2,iVar6,
              *(undefined1 *)((uint)DAT_00002434 * 0x32 + 0x100 + (uint)DAT_00002435 * 3000 + iVar6)
             );
    iVar6 = iVar7;
  } while (iVar7 != 0x32);
  return;
}

