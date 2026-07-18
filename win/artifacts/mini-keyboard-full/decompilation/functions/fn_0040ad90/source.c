
/* WARNING: Type propagation algorithm not settling */

undefined4 __fastcall FUN_0040ad90(int param_1)

{
  undefined1 *puVar1;
  undefined4 uVar2;
  byte bVar3;
  uint uVar4;
  int iVar5;
  undefined1 *puVar6;
  undefined4 *puVar7;
  int iVar8;
  char *pcVar9;
  int local_90;
  int local_88;
  int local_80;
  char local_74 [16];
  char *local_64;
  undefined1 local_5d [3];
  undefined1 uStack_5a;
  undefined4 local_20;
  undefined1 local_1c [12];
  
  local_20 = 0;
  puVar7 = (undefined4 *)((int)local_5d + 1U);
  for (uVar4 = (uint)(local_1c + -((int)local_5d + 1U)) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    *puVar7 = 0;
    puVar7 = puVar7 + 1;
  }
  _local_5d = 3;
  local_90 = 0;
  local_80 = 0;
  do {
    iVar5 = 0;
    iVar8 = local_80;
    do {
      if ((&DAT_0061e080)[iVar5 + local_90] == '\x01') {
        iVar5 = 0;
        do {
          iVar5 = iVar5 + 1;
          local_5d[iVar5] = *(undefined1 *)(iVar8 + 0x61e13f + iVar5);
        } while (iVar5 != 0x32);
        local_88 = hid_write(*(undefined4 *)(param_1 + 0x968),local_5d,0x41);
        local_74[0] = '\x02';
        local_74[1] = '\0';
        local_74[2] = '\0';
        local_74[3] = '\0';
        local_74[4] = '\0';
        local_74[5] = '\0';
        local_74[6] = '\0';
        local_74[7] = '\0';
        local_74[8] = '\0';
        local_74[9] = '\0';
        local_74[10] = '\0';
        local_74[0xb] = '\0';
        local_74[0xc] = '\0';
        local_74[0xd] = '\0';
        local_74[0xe] = '\0';
        local_74[0xf] = '\0';
        local_64 = "default";
        iVar5 = local_80;
        if (local_88 < 0) {
          hid_error(*(undefined4 *)(param_1 + 0x968));
          QMessageLogger::debug(local_74);
          bVar3 = (byte)local_90 & 1;
        }
        else {
          QMessageLogger::debug(local_74);
          bVar3 = 1;
          local_90 = 1;
        }
      }
      else {
        bVar3 = 0;
      }
      puVar1 = (undefined1 *)((int)local_5d + 1);
      puVar6 = puVar1;
      do {
        *puVar6 = 0;
        puVar6 = puVar6 + 1;
      } while (puVar6 != local_1c);
      if ((0x3a < (byte)iVar5) && (bVar3 != 0)) {
        _local_5d = CONCAT13(0xff,CONCAT21(0xfefd,local_5d[0]));
        local_88 = hid_write(*(undefined4 *)(param_1 + 0x968),local_5d,0x41);
        do {
          *puVar1 = 0;
          puVar1 = puVar1 + 1;
        } while (puVar1 != puVar6);
        Sleep(200);
      }
      iVar5 = iVar5 + 1;
      iVar8 = iVar8 + 0x32;
    } while (iVar5 != 0x3c);
    local_90 = local_90 + 0x3c;
    local_80 = local_80 + 3000;
  } while (local_90 != 0xb4);
  if (local_88 < 0) {
    local_74[0] = '\x02';
    local_74[1] = '\0';
    local_74[2] = '\0';
    local_74[3] = '\0';
    local_74[4] = '\0';
    local_74[5] = '\0';
    local_74[6] = '\0';
    local_74[7] = '\0';
    local_74[8] = '\0';
    local_74[9] = '\0';
    local_74[10] = '\0';
    local_74[0xb] = '\0';
    local_74[0xc] = '\0';
    local_74[0xd] = '\0';
    local_74[0xe] = '\0';
    local_74[0xf] = '\0';
    local_64 = "default";
    uVar2 = hid_error(*(undefined4 *)(param_1 + 0x968));
    pcVar9 = "err_string = %ls\n";
    QMessageLogger::debug(local_74);
    FUN_0040abd0(pcVar9,uVar2);
  }
  else {
    FUN_0040a900();
    local_74[0] = '\x02';
    local_74[1] = '\0';
    local_74[2] = '\0';
    local_74[3] = '\0';
    local_74[4] = '\0';
    local_74[5] = '\0';
    local_74[6] = '\0';
    local_74[7] = '\0';
    local_74[8] = '\0';
    local_74[9] = '\0';
    local_74[10] = '\0';
    local_74[0xb] = '\0';
    local_74[0xc] = '\0';
    local_74[0xd] = '\0';
    local_74[0xe] = '\0';
    local_74[0xf] = '\0';
    local_64 = "default";
    QMessageLogger::debug(local_74);
  }
  puVar1 = &DAT_0061e080;
  do {
    puVar6 = puVar1 + 0x3c;
    do {
      *puVar1 = 0;
      puVar1 = puVar1 + 1;
    } while (puVar1 != puVar6);
  } while (puVar1 != &DAT_0061e134);
  return 0;
}

