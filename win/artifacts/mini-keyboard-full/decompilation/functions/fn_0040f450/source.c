
/* WARNING: Type propagation algorithm not settling */

void __thiscall FUN_0040f450(int param_1,byte param_2,byte param_3,byte param_4)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  undefined4 *puVar5;
  char *pcVar6;
  undefined4 uVar7;
  byte local_c0;
  byte bStack_be;
  undefined2 uStack_9e;
  byte local_9c;
  byte local_9b;
  byte local_9a;
  undefined1 local_5d [2];
  undefined1 auStack_5b [59];
  undefined4 local_20;
  undefined1 auStack_1c [12];
  
  _local_5d = 0;
  local_20 = 0;
  local_9c = param_3;
  puVar5 = (undefined4 *)((int)local_5d + 1U);
  for (uVar3 = (uint)(auStack_1c + -((int)local_5d + 1U)) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *puVar5 = 0;
    puVar5 = puVar5 + 1;
  }
  local_9b = param_4;
  uStack_9e = 0xfa03;
  if (param_2 != 0) {
    bStack_be = 0;
    uVar3 = (uint)param_4 * 3 + 0xf;
    do {
      while( true ) {
        bStack_be = bStack_be + 1;
        local_9a = bStack_be;
        iVar2 = hid_write(*(undefined4 *)(param_1 + 0x968),&uStack_9e,0x41);
        if (iVar2 < 0) break;
        local_c0 = 1;
        if (param_3 != 0) {
          do {
            uVar7 = 0x40;
            iVar2 = hid_read(*(undefined4 *)(param_1 + 0x968),local_5d,0x40);
            if (iVar2 < 0) {
              pcVar6 = (char *)0x40f7ea;
              uVar7 = hid_error(*(undefined4 *)(param_1 + 0x968));
              QMessageLogger::debug(pcVar6);
            }
            pcVar6 = (char *)0x40f574;
            hid_set_nonblocking(*(undefined4 *)(param_1 + 0x968),0,uVar7);
            iVar2 = 0;
            do {
              iVar1 = iVar2 + 1;
              (&DAT_0061e141)[iVar2 + (uint)local_c0 * 0x32 + (local_9a - 1) * 3000] =
                   auStack_5b[iVar2];
              QMessageLogger::debug(pcVar6);
              iVar2 = iVar1;
            } while (iVar1 != 0x31);
            local_c0 = local_c0 + 1;
          } while (local_c0 <= param_3);
        }
        if (uVar3 != 0xf) {
          local_c0 = 0x10;
          uVar4 = 0x10;
          do {
            uVar7 = 0x40;
            iVar2 = hid_read(*(undefined4 *)(param_1 + 0x968),local_5d,0x40);
            if (iVar2 < 0) {
              pcVar6 = (char *)0x40f796;
              uVar7 = hid_error(*(undefined4 *)(param_1 + 0x968));
              QMessageLogger::debug(pcVar6);
            }
            pcVar6 = (char *)0x40f687;
            hid_set_nonblocking(*(undefined4 *)(param_1 + 0x968),0,uVar7);
            iVar2 = 0;
            do {
              iVar1 = iVar2 + 1;
              (&DAT_0061e141)[iVar2 + uVar4 * 0x32 + (local_9a - 1) * 3000] = auStack_5b[iVar2];
              QMessageLogger::debug(pcVar6);
              iVar2 = iVar1;
            } while (iVar1 != 0x31);
            local_c0 = local_c0 + 1;
            uVar4 = (uint)local_c0;
          } while (uVar4 <= uVar3);
        }
        if (bStack_be == param_2) goto LAB_0040f739;
      }
      pcVar6 = (char *)0x40f83e;
      hid_error(*(undefined4 *)(param_1 + 0x968));
      QMessageLogger::debug(pcVar6);
    } while (bStack_be != param_2);
  }
LAB_0040f739:
  FUN_0040e310(DAT_00620475);
  return;
}

