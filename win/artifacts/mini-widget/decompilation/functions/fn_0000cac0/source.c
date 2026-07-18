
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::Traversal_Key_Txt(unsigned char) */

void Widget::Traversal_Key_Txt(uchar param_1)

{
  char cVar1;
  byte bVar2;
  uint uVar3;
  int in_ECX;
  byte *pbVar4;
  int *piVar5;
  int *piVar6;
  undefined **ppuVar7;
  char *pcVar8;
  undefined *local_6c;
  undefined4 local_68;
  code *local_58;
  byte local_52;
  char local_51;
  byte *local_50;
  int *local_44;
  int *local_40;
  int *local_3c;
  int *local_38;
  int *local_34;
  int *local_30;
  int *local_2c;
  int *local_28;
  int *local_24;
  int *local_20 [4];
  
  ppuVar7 = &local_6c;
  local_68 = 0;
  local_6c = &DAT_0000254c;
  local_58 = ___imp___ZN7QString16fromAscii_helperEPKci;
  local_44 = (int *)(*___imp___ZN7QString16fromAscii_helperEPKci)();
  local_51 = '\x02';
  pcVar8 = (char *)((uint)param_1 * 3000 + 0x135);
  do {
    local_50 = (byte *)(pcVar8 + 0x2b);
    pbVar4 = (byte *)(pcVar8 + 7);
    local_52 = local_51 - 1;
    cVar1 = *pcVar8;
LAB_0000cb2f:
    do {
      if (cVar1 == '\x01') {
        bVar2 = *pbVar4;
        if (bVar2 != 0) {
          piVar5 = (int *)ppuVar7;
          if ((bVar2 & 1) != 0) {
            ppuVar7[2] = (undefined *)0x5;
            ppuVar7[1] = (undefined *)0x22;
            *ppuVar7 = (undefined *)&local_40;
            ppuVar7[-1] = (undefined *)0xcedc;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *ppuVar7 = (undefined *)&local_40;
            ppuVar7[-1] = (undefined *)0xceeb;
            (*___imp___ZN7QString6appendERKS_)();
            piVar5 = (int *)(ppuVar7 + -1);
            if (*local_40 == 0) {
LAB_0000d1f0:
              ppuVar7[1] = (undefined *)0x4;
              *ppuVar7 = (undefined *)0x2;
              ppuVar7[-1] = (undefined *)local_40;
              ppuVar7[-2] = (undefined *)0xd209;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            }
            else if (*local_40 != -1) {
              LOCK();
              *local_40 = *local_40 + -1;
              UNLOCK();
              if (*local_40 == 0) goto LAB_0000d1f0;
            }
            bVar2 = *pbVar4;
          }
          piVar6 = piVar5;
          if ((bVar2 & 2) != 0) {
            piVar5[2] = 6;
            piVar5[1] = 0x28;
            *piVar5 = (int)&local_3c;
            piVar5[-1] = 0xce81;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *piVar5 = (int)&local_3c;
            piVar5[-1] = 0xce90;
            (*___imp___ZN7QString6appendERKS_)();
            piVar6 = piVar5 + -1;
            if (*local_3c == 0) {
LAB_0000d230:
              piVar5[1] = 4;
              *piVar5 = 2;
              piVar5[-1] = (int)local_3c;
              piVar5[-2] = 0xd249;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            }
            else if (*local_3c != -1) {
              LOCK();
              *local_3c = *local_3c + -1;
              UNLOCK();
              if (*local_3c == 0) goto LAB_0000d230;
            }
            bVar2 = *pbVar4;
          }
          piVar5 = piVar6;
          if ((bVar2 & 4) != 0) {
            piVar6[2] = 4;
            piVar6[1] = 0x34;
            *piVar6 = (int)&local_38;
            piVar6[-1] = 0xce2c;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *piVar6 = (int)&local_38;
            piVar6[-1] = 0xce3b;
            (*___imp___ZN7QString6appendERKS_)();
            piVar5 = piVar6 + -1;
            if (*local_38 == 0) {
LAB_0000d290:
              piVar6[1] = 4;
              *piVar6 = 2;
              piVar6[-1] = (int)local_38;
              piVar6[-2] = 0xd2a9;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            }
            else if (*local_38 != -1) {
              LOCK();
              *local_38 = *local_38 + -1;
              UNLOCK();
              if (*local_38 == 0) goto LAB_0000d290;
            }
            bVar2 = *pbVar4;
          }
          if ((bVar2 & 8) != 0) {
            piVar5[2] = 4;
            piVar5[1] = 0x2f;
            *piVar5 = (int)&local_34;
            piVar5[-1] = 0xcd1f;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *piVar5 = (int)&local_34;
            piVar5[-1] = 0xcd2e;
            (*___imp___ZN7QString6appendERKS_)();
            if (*local_34 == 0) {
LAB_0000d2b0:
              piVar5[1] = 4;
              *piVar5 = 2;
              piVar5[-1] = (int)local_34;
              piVar5[-2] = 0xd2c9;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            }
            else if (*local_34 != -1) {
              LOCK();
              *local_34 = *local_34 + -1;
              UNLOCK();
              if (*local_34 == 0) goto LAB_0000d2b0;
            }
            bVar2 = *pbVar4;
            piVar5 = piVar5 + -1;
          }
          if ((bVar2 & 0x10) != 0) {
            piVar5[2] = 0xb;
            piVar5[1] = 0x1ed;
            *piVar5 = (int)&local_30;
            piVar5[-1] = 0xcd77;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *piVar5 = (int)&local_30;
            piVar5[-1] = 0xcd86;
            (*___imp___ZN7QString6appendERKS_)();
            if (*local_30 == 0) {
LAB_0000d250:
              piVar5[1] = 4;
              *piVar5 = 2;
              piVar5[-1] = (int)local_30;
              piVar5[-2] = 0xd269;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            }
            else if (*local_30 != -1) {
              LOCK();
              *local_30 = *local_30 + -1;
              UNLOCK();
              if (*local_30 == 0) goto LAB_0000d250;
            }
            bVar2 = *pbVar4;
            piVar5 = piVar5 + -1;
          }
          piVar6 = piVar5;
          if ((bVar2 & 0x20) != 0) {
            piVar5[2] = 0xc;
            piVar5[1] = 0x1d5;
            *piVar5 = (int)&local_2c;
            piVar5[-1] = 0xcdcf;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *piVar5 = (int)&local_2c;
            piVar5[-1] = 0xcdde;
            (*___imp___ZN7QString6appendERKS_)();
            piVar6 = piVar5 + -1;
            if (*local_2c == 0) {
LAB_0000d270:
              piVar5[1] = 4;
              *piVar5 = 2;
              piVar5[-1] = (int)local_2c;
              piVar5[-2] = 0xd289;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            }
            else if (*local_2c != -1) {
              LOCK();
              *local_2c = *local_2c + -1;
              UNLOCK();
              if (*local_2c == 0) goto LAB_0000d270;
            }
            bVar2 = *pbVar4;
          }
          if ((bVar2 & 0x40) != 0) {
            piVar6[2] = 10;
            piVar6[1] = 0x1e2;
            *piVar6 = (int)&local_28;
            piVar6[-1] = 0xcc5c;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *piVar6 = (int)&local_28;
            piVar6[-1] = 0xcc6b;
            (*___imp___ZN7QString6appendERKS_)();
            if (*local_28 == 0) {
LAB_0000d210:
              piVar6[1] = 4;
              *piVar6 = 2;
              piVar6[-1] = (int)local_28;
              piVar6[-2] = 0xd229;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            }
            else if (*local_28 != -1) {
              LOCK();
              *local_28 = *local_28 + -1;
              UNLOCK();
              if (*local_28 == 0) goto LAB_0000d210;
            }
            bVar2 = *pbVar4;
            piVar6 = piVar6 + -1;
          }
          ppuVar7 = (undefined **)piVar6;
          if ((char)bVar2 < '\0') {
            piVar6[2] = 10;
            piVar6[1] = 0x1f9;
            *piVar6 = (int)&local_24;
            piVar6[-1] = 0xccb4;
            (*___imp___ZN7QString15fromUtf8_helperEPKci)();
            *piVar6 = (int)&local_24;
            piVar6[-1] = 0xccc3;
            (*___imp___ZN7QString6appendERKS_)();
            if (*local_24 == 0) {
LAB_0000cce5:
              piVar6[1] = 4;
              *piVar6 = 2;
              piVar6[-1] = (int)local_24;
              piVar6[-2] = 0xccfe;
              (*___imp___ZN10QArrayData10deallocateEPS_jj)();
              ppuVar7 = (undefined **)(piVar6 + -1);
            }
            else {
              ppuVar7 = (undefined **)(piVar6 + -1);
              if (*local_24 != -1) {
                LOCK();
                *local_24 = *local_24 + -1;
                UNLOCK();
                ppuVar7 = (undefined **)(piVar6 + -1);
                if (*local_24 == 0) goto LAB_0000cce5;
              }
            }
          }
        }
        if (pbVar4[1] != 0) {
          uVar3 = (uint)*(byte *)(in_ECX + 0x4c);
          if (uVar3 < *(byte *)(in_ECX + 0x4d)) {
            do {
              if (pbVar4[1] == *(byte *)(in_ECX + 0x800 + uVar3)) {
                if (*(int *)(in_ECX + 0x2c) == 1) {
                  *ppuVar7 = (undefined *)(in_ECX + 0x494 + uVar3 * 4);
                  ppuVar7[-1] = (undefined *)0xd304;
                  (*___imp___ZN7QString6appendERKS_)();
                }
                else {
                  *ppuVar7 = (undefined *)(in_ECX + 0x128 + uVar3 * 4);
                  ppuVar7[-1] = (undefined *)0xcbbd;
                  (*___imp___ZN7QString6appendERKS_)();
                }
                piVar5 = (int *)(ppuVar7 + -1);
                ppuVar7 = ppuVar7 + -1;
                pbVar4 = pbVar4 + 2;
                cVar1 = *pcVar8;
                if (local_50 == pbVar4) goto LAB_0000cbd0;
                goto LAB_0000cb2f;
              }
              uVar3 = uVar3 + 1;
            } while (uVar3 != *(byte *)(in_ECX + 0x4d));
          }
        }
        cVar1 = *pcVar8;
      }
      pbVar4 = pbVar4 + 2;
      piVar5 = (int *)ppuVar7;
    } while (local_50 != pbVar4);
LAB_0000cbd0:
    ppuVar7 = (undefined **)piVar5;
    if (cVar1 == '\x02') {
      if (pcVar8[7] != '\0') {
        uVar3 = (uint)*(byte *)(in_ECX + 0x46);
        if (uVar3 <= *(byte *)(in_ECX + 0x47)) {
          do {
            if (pcVar8[7] == *(char *)(in_ECX + 0x800 + uVar3)) goto LAB_0000d344;
            uVar3 = uVar3 + 1;
          } while ((int)uVar3 <= (int)(uint)*(byte *)(in_ECX + 0x47));
        }
      }
    }
    else if (cVar1 == '\x03') {
      cVar1 = pcVar8[8];
      if (cVar1 == '\x01') {
        if ((pcVar8[0xb] == '\0') && (pcVar8[7] == '\0')) {
          uVar3 = (uint)*(byte *)(in_ECX + 0x4a);
          if (*(int *)(in_ECX + 0x2c) == 1) {
LAB_0000d380:
            *piVar5 = in_ECX + 0x494 + uVar3 * 4;
            piVar5[-1] = 0xd393;
            (*___imp___ZN7QString6appendERKS_)();
          }
          else {
LAB_0000d34a:
            *piVar5 = in_ECX + 0x128 + uVar3 * 4;
            piVar5[-1] = 0xd35d;
            (*___imp___ZN7QString6appendERKS_)();
          }
          goto LAB_0000cc25;
        }
      }
      else if (cVar1 == '\x04') {
        if ((pcVar8[0xb] == '\0') && (pcVar8[7] == '\0')) {
          if (*(int *)(in_ECX + 0x2c) == 1) {
            *piVar5 = in_ECX + 0x498 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
            piVar5[-1] = 0xd5ad;
            (*___imp___ZN7QString6appendERKS_)();
          }
          else {
            *piVar5 = in_ECX + 300 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
            piVar5[-1] = 0xd3cd;
            (*___imp___ZN7QString6appendERKS_)();
          }
          goto LAB_0000cc25;
        }
      }
      else if (cVar1 == '\x02') {
        if ((pcVar8[0xb] == '\0') && (pcVar8[7] == '\0')) {
          if (*(int *)(in_ECX + 0x2c) == 1) {
            *piVar5 = in_ECX + 0x49c + (uint)*(byte *)(in_ECX + 0x4a) * 4;
            piVar5[-1] = 0xd572;
            (*___imp___ZN7QString6appendERKS_)();
          }
          else {
            *piVar5 = in_ECX + 0x130 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
            piVar5[-1] = 0xcc25;
            (*___imp___ZN7QString6appendERKS_)();
          }
          goto LAB_0000cc25;
        }
      }
      else if (cVar1 == '\0') {
        if (pcVar8[0xb] == '\x01') {
          if (pcVar8[7] == '\0') {
            uVar3 = *(byte *)(in_ECX + 0x4a) + 3;
LAB_0000d344:
            if (*(int *)(in_ECX + 0x2c) == 1) goto LAB_0000d380;
            goto LAB_0000d34a;
          }
          cVar1 = pcVar8[7];
          if (cVar1 == '\x01') {
            if (*(int *)(in_ECX + 0x2c) == 1) {
              *piVar5 = in_ECX + 0x4a8 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xdb0b;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *piVar5 = in_ECX + 0x13c + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd50e;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else if (cVar1 == '\x02') {
            if (*(int *)(in_ECX + 0x2c) == 1) {
              *piVar5 = in_ECX + 0x4b0 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd5dd;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *piVar5 = in_ECX + 0x144 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd534;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else {
            if (cVar1 != '\x04') goto LAB_0000cc28;
            if (*(int *)(in_ECX + 0x2c) == 1) {
              *piVar5 = in_ECX + 0x4b8 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd5c5;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *piVar5 = in_ECX + 0x14c + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd4b6;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
        }
        else {
          if (pcVar8[0xb] != -1) goto LAB_0000cc28;
          cVar1 = pcVar8[7];
          if (cVar1 == '\0') {
            if (*(int *)(in_ECX + 0x2c) == 1) {
              *piVar5 = in_ECX + 0x4a4 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xdaf3;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *piVar5 = in_ECX + 0x138 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd43c;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else if (cVar1 == '\x01') {
            if (*(int *)(in_ECX + 0x2c) == 1) {
              *piVar5 = in_ECX + 0x4ac + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd60d;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *piVar5 = in_ECX + 0x140 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd478;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else if (cVar1 == '\x02') {
            if (*(int *)(in_ECX + 0x2c) == 1) {
              *piVar5 = in_ECX + 0x4b4 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd5f5;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *piVar5 = in_ECX + 0x148 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd55a;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else {
            if (cVar1 != '\x04') goto LAB_0000cc28;
            if (*(int *)(in_ECX + 0x2c) == 1) {
              *piVar5 = in_ECX + 0x4bc + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xdadb;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *piVar5 = in_ECX + 0x150 + (uint)*(byte *)(in_ECX + 0x4a) * 4;
              piVar5[-1] = 0xd4e8;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
        }
LAB_0000cc25:
        ppuVar7 = (undefined **)(piVar5 + -1);
      }
    }
LAB_0000cc28:
    if (local_52 < 0x19) {
                    /* WARNING: Could not emulate address calculation at 0x0000cc34 */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)((uint)local_52 * 4 + 0x2664))();
      return;
    }
    ppuVar7[2] = (undefined *)0x0;
    ppuVar7[1] = &DAT_0000254c;
    *ppuVar7 = (undefined *)local_20;
    ppuVar7[-1] = (undefined *)0xcf4f;
    (*___imp___ZN7QString15fromUtf8_helperEPKci)();
    piVar6 = local_20[0];
    piVar5 = local_44;
    local_20[0] = local_44;
    local_44 = piVar6;
    if (*piVar5 == 0) {
LAB_0000d2d0:
      ppuVar7[2] = (undefined *)0x4;
      ppuVar7[1] = (undefined *)0x2;
      *ppuVar7 = (undefined *)piVar5;
      ppuVar7[-1] = (undefined *)0xd2ec;
      (*___imp___ZN10QArrayData10deallocateEPS_jj)();
    }
    else if (*piVar5 != -1) {
      LOCK();
      *piVar5 = *piVar5 + -1;
      UNLOCK();
      if (*piVar5 == 0) goto LAB_0000d2d0;
    }
    piVar5 = local_44;
    if ((((pcVar8[8] == '\0') && (pcVar8[0xb] == '\0')) && (pcVar8[7] == '\0')) && (local_52 < 0x19)
       ) {
                    /* WARNING: Could not emulate address calculation at 0x0000cf9e */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)((uint)local_52 * 4 + 0x26c8))();
      return;
    }
    if (local_51 == '<') break;
    pcVar8 = pcVar8 + 0x32;
    local_51 = local_51 + '\x01';
  } while( true );
  if (*local_44 != 0) {
    if (*local_44 != -1) {
      LOCK();
      *local_44 = *local_44 + -1;
      UNLOCK();
      if (*local_44 == 0) goto LAB_0000d577;
    }
    return;
  }
LAB_0000d577:
  ppuVar7[2] = (undefined *)0x4;
  ppuVar7[1] = (undefined *)0x2;
  *ppuVar7 = (undefined *)piVar5;
  ppuVar7[-1] = (undefined *)0xd590;
  (*___imp___ZN10QArrayData10deallocateEPS_jj)();
  return;
}

