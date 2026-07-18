
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::HID_write() */

undefined4 Widget::HID_write(void)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  byte bVar6;
  uint uVar7;
  int iVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  int iVar13;
  int aiStack_b0 [7];
  byte local_94 [4];
  int local_90 [7];
  undefined4 local_74 [5];
  undefined1 local_5d [3];
  undefined1 auStack_5a [58];
  undefined4 local_20;
  undefined1 local_1c [12];
  
  puVar11 = aiStack_b0 + 1;
  local_20 = 0;
  puVar10 = (undefined4 *)((int)local_5d + 1U);
  for (uVar7 = (uint)(local_1c + -((int)local_5d + 1U)) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
    *puVar10 = 0;
    puVar10 = puVar10 + 1;
  }
  _local_5d = 3;
  local_90[0] = 0;
  local_90[4] = 0;
  local_94[0] = 0;
  local_94[1] = 0;
  local_94[2] = 0;
  local_94[3] = 0;
  do {
    iVar13 = puVar11[0xb];
    iVar8 = 0;
    puVar10 = puVar11;
    do {
      if (*(char *)(puVar10[7] + 0x40 + iVar8) == '\x01') {
        iVar3 = 0;
        do {
          iVar3 = iVar3 + 1;
          local_5d[iVar3] = *(undefined1 *)(iVar13 + 0xff + iVar3);
        } while (iVar3 != 0x32);
        puVar10[2] = 0x41;
        puVar10[1] = local_5d;
        puVar10[10] = iVar8;
        *puVar10 = *(undefined4 *)(puVar10[8] + 0x968);
        puVar10[-1] = 0x96ba;
        iVar8 = _hid_write();
        puVar10[9] = iVar8;
        puVar10[0xe] = 2;
        puVar10[0xf] = 0;
        puVar10[0x10] = 0;
        puVar10[0x11] = 0;
        puVar10[0x12] = &DAT_000022f1;
        if (iVar8 < 0) {
          *puVar10 = *(undefined4 *)(puVar10[8] + 0x968);
          puVar10[-1] = 0x97a1;
          uVar5 = _hid_error();
          puVar10[2] = uVar5;
          puVar10[1] = &DAT_000022f9;
          *puVar10 = puVar10 + 0xe;
          puVar10[-1] = 0x97ba;
          (*___imp___ZNK14QMessageLogger5debugEPKcz)();
          iVar8 = puVar10[10];
          bVar6 = *(byte *)(puVar10 + 6) & 1;
        }
        else {
          puVar10[2] = puVar10[10];
          puVar10[10] = puVar10[10];
          puVar10[1] = &DAT_00002523;
          *puVar10 = puVar10 + 0xe;
          puVar10[-1] = 0x970f;
          (*___imp___ZNK14QMessageLogger5debugEPKcz)();
          bVar6 = 1;
          puVar10[6] = 1;
          iVar8 = puVar10[10];
        }
      }
      else {
        bVar6 = *(byte *)(puVar10 + 6) & 1;
      }
      puVar12 = puVar10 + 0x14;
      puVar9 = puVar12;
      do {
        *(undefined1 *)puVar9 = 0;
        puVar9 = (undefined4 *)((int)puVar9 + 1);
      } while (puVar9 != puVar10 + 0x24);
      puVar11 = puVar10;
      if ((0x3a < (byte)iVar8) && (bVar6 != 0)) {
        puVar10[2] = 0x41;
        puVar10[1] = local_5d;
        puVar10[6] = iVar8;
        *(undefined2 *)(puVar10 + 0x14) = 0xfefd;
        *(undefined1 *)((int)puVar10 + 0x52) = 0xff;
        *puVar10 = *(undefined4 *)(puVar10[8] + 0x968);
        puVar10[-1] = 0x9756;
        uVar4 = _hid_write();
        uVar5 = puVar10[6];
        puVar10[9] = uVar4;
        do {
          *(undefined1 *)puVar12 = 0;
          puVar12 = (undefined4 *)((int)puVar12 + 1);
        } while (puVar12 != puVar9);
        puVar10[10] = uVar5;
        *puVar10 = 200;
        puVar10[-1] = 0x977b;
        (*___imp__Sleep_4)();
        puVar11 = puVar10 + -1;
        puVar10[5] = 0;
        iVar8 = puVar10[9];
      }
      iVar8 = iVar8 + 1;
      iVar13 = iVar13 + 0x32;
      puVar10 = puVar11;
    } while (iVar8 != 0x3c);
    puVar11[7] = puVar11[7] + 0x3c;
    puVar11[0xb] = puVar11[0xb] + 3000;
  } while (puVar11[7] != 0xb4);
  if ((int)puVar11[9] < 0) {
    puVar11[0xe] = 2;
    puVar11[0xf] = 0;
    puVar11[0x10] = 0;
    puVar11[0x11] = 0;
    puVar11[0x12] = &DAT_000022f1;
    *puVar11 = *(undefined4 *)(puVar11[8] + 0x968);
    puVar11[-1] = 0x9805;
    uVar5 = _hid_error();
    puVar11[2] = uVar5;
    puVar11[1] = &DAT_000022f9;
    *puVar11 = puVar11 + 0xe;
    puVar11[-1] = 0x981e;
    (*___imp___ZNK14QMessageLogger5debugEPKcz)();
    puVar11[-1] = 0x9825;
    Display_Dev_Disconnect();
  }
  else {
    puVar11[-1] = 0x961f;
    Display_Opt_Inf();
    puVar11[0xe] = 2;
    puVar11[0xf] = 0;
    puVar11[0x10] = 0;
    puVar11[0x11] = 0;
    puVar11[0x12] = &DAT_000022f1;
    puVar11[1] = &DAT_0000253b;
    *puVar11 = puVar11 + 0xe;
    puVar11[-1] = 0x965c;
    (*___imp___ZNK14QMessageLogger5debugEPKcz)();
  }
  puVar2 = &DAT_00000040;
  do {
    puVar1 = puVar2 + 0x3c;
    do {
      *puVar2 = 0;
      puVar2 = puVar2 + 1;
    } while (puVar2 != puVar1);
  } while (puVar2 != (undefined1 *)0xf4);
  return 0;
}

