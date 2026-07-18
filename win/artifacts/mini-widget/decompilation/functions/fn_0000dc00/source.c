
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::read_Hidkey_Data(unsigned char, unsigned char, unsigned char) */

void Widget::read_Hidkey_Data(uchar param_1,uchar param_2,uchar param_3)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  int in_ECX;
  uint uVar4;
  uint uVar5;
  undefined4 *puVar6;
  byte local_c0;
  byte local_be;
  undefined4 local_b4;
  undefined4 local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined *local_a4;
  undefined2 local_9e;
  uchar local_9c;
  byte local_9b;
  byte local_9a;
  undefined1 local_5d [2];
  undefined1 auStack_5b [59];
  undefined4 local_20;
  undefined1 auStack_1c [12];
  
  _local_5d = 0;
  local_20 = 0;
  puVar6 = (undefined4 *)((int)local_5d + 1U);
  for (uVar4 = (uint)(auStack_1c + -((int)local_5d + 1U)) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    *puVar6 = 0;
    puVar6 = puVar6 + 1;
  }
  local_9c = param_2;
  local_9b = param_3;
  local_9e = 0xfa03;
  if (param_1 != '\0') {
    local_be = 0;
    uVar4 = (uint)param_3 * 3 + 0xf;
    do {
      while( true ) {
        local_be = local_be + 1;
        local_9a = local_be;
        iVar2 = _hid_write(*(undefined4 *)(in_ECX + 0x968),&local_9e,0x41);
        if (iVar2 < 0) break;
        local_c0 = 1;
        if (param_2 != '\0') {
          do {
            iVar2 = _hid_read(*(undefined4 *)(in_ECX + 0x968),local_5d,0x40);
            if (iVar2 < 0) {
              local_b4 = 2;
              local_b0 = 0;
              local_ac = 0;
              local_a8 = 0;
              local_a4 = &DAT_000022f1;
              uVar3 = _hid_error(*(undefined4 *)(in_ECX + 0x968));
              (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_b4,&DAT_000022f9,uVar3);
            }
            _hid_set_nonblocking(*(undefined4 *)(in_ECX + 0x968),0);
            iVar2 = 0;
            do {
              iVar1 = iVar2 + 1;
              local_b4 = 2;
              local_b0 = 0;
              local_ac = 0;
              local_a8 = 0;
              local_a4 = &DAT_000022f1;
              *(undefined1 *)((uint)local_c0 * 0x32 + (local_9a - 1) * 3000 + 0x101 + iVar2) =
                   auStack_5b[iVar2];
              (*___imp___ZNK14QMessageLogger5debugEPKcz)
                        (&local_b4,&DAT_0000272c,iVar2,
                         *(undefined1 *)
                          ((uint)local_c0 * 0x32 + 0x100 + (local_9b - 1) * 3000 + iVar2));
              iVar2 = iVar1;
            } while (iVar1 != 0x31);
            local_c0 = local_c0 + 1;
          } while (local_c0 <= param_2);
        }
        if (uVar4 != 0xf) {
          local_c0 = 0x10;
          uVar5 = 0x10;
          do {
            iVar2 = _hid_read(*(undefined4 *)(in_ECX + 0x968),local_5d,0x40);
            if (iVar2 < 0) {
              local_b4 = 2;
              local_b0 = 0;
              local_ac = 0;
              local_a8 = 0;
              local_a4 = &DAT_000022f1;
              uVar3 = _hid_error(*(undefined4 *)(in_ECX + 0x968));
              (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_b4,&DAT_000022f9,uVar3);
            }
            _hid_set_nonblocking(*(undefined4 *)(in_ECX + 0x968),0);
            iVar2 = 0;
            do {
              iVar1 = iVar2 + 1;
              local_b4 = 2;
              local_b0 = 0;
              local_ac = 0;
              local_a8 = 0;
              local_a4 = &DAT_000022f1;
              *(undefined1 *)(uVar5 * 0x32 + (local_9a - 1) * 3000 + 0x101 + iVar2) =
                   auStack_5b[iVar2];
              (*___imp___ZNK14QMessageLogger5debugEPKcz)
                        (&local_b4,&DAT_0000272c,iVar2,
                         *(undefined1 *)(uVar5 * 0x32 + 0x100 + (local_9b - 1) * 3000 + iVar2));
              iVar2 = iVar1;
            } while (iVar1 != 0x31);
            local_c0 = local_c0 + 1;
            uVar5 = (uint)local_c0;
          } while (uVar5 <= uVar4);
        }
        if (local_be == param_1) goto LAB_0000dee9;
      }
      local_b4 = 2;
      local_b0 = 0;
      local_ac = 0;
      local_a8 = 0;
      local_a4 = &DAT_000022f1;
      uVar3 = _hid_error(*(undefined4 *)(in_ECX + 0x968));
      (*___imp___ZNK14QMessageLogger5debugEPKcz)(&local_b4,&DAT_000022f9,uVar3);
    } while (local_be != param_1);
  }
LAB_0000dee9:
  Traversal_Key_Txt(DAT_00002435);
  return;
}

