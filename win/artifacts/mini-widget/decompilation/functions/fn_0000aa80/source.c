
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::Dsp_Select_PHY_KeValue() */

void Widget::Dsp_Select_PHY_KeValue(void)

{
  int iVar1;
  code *pcVar2;
  uint uVar3;
  char cVar4;
  byte bVar5;
  int in_ECX;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  int *piVar9;
  int *piVar10;
  undefined **ppuVar11;
  uint uVar12;
  undefined *local_6c;
  undefined4 local_68;
  uint local_5c;
  uint local_58;
  int local_54;
  int local_50;
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
  
  uVar12 = 0xb;
  ppuVar11 = &local_6c;
  local_68 = 0;
  local_6c = &DAT_0000254c;
  local_54 = in_ECX;
  local_44 = (int *)(*___imp___ZN7QString16fromAscii_helperEPKci)();
  uVar7 = (uint)DAT_00002434;
  uVar3 = (uint)DAT_00002435;
  cVar4 = *(char *)(uVar7 * 0x32 + 0x103 + uVar3 * 3000);
  do {
    if (cVar4 == '\x01') {
      iVar1 = uVar12 - 1;
      bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
      if (bVar5 != 0) {
        piVar9 = (int *)ppuVar11;
        if ((bVar5 & 1) != 0) {
          ppuVar11[2] = (undefined *)0x5;
          ppuVar11[1] = (undefined *)0x22;
          *ppuVar11 = (undefined *)&local_40;
          ppuVar11[-1] = (undefined *)0xaf70;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *ppuVar11 = (undefined *)&local_40;
          ppuVar11[-1] = (undefined *)0xaf7c;
          (*___imp___ZN7QString6appendERKS_)();
          piVar9 = (int *)(ppuVar11 + -1);
          if (*local_40 == 0) {
LAB_0000b074:
            ppuVar11[1] = (undefined *)0x4;
            *ppuVar11 = (undefined *)0x2;
            ppuVar11[-1] = (undefined *)local_40;
            ppuVar11[-2] = (undefined *)0xb08d;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
          }
          else if (*local_40 != -1) {
            LOCK();
            *local_40 = *local_40 + -1;
            UNLOCK();
            if (*local_40 == 0) goto LAB_0000b074;
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
        }
        piVar10 = piVar9;
        if ((bVar5 & 2) != 0) {
          piVar9[2] = 6;
          piVar9[1] = 0x28;
          *piVar9 = (int)&local_3c;
          piVar9[-1] = 0xaeff;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *piVar9 = (int)&local_3c;
          piVar9[-1] = 0xaf0b;
          (*___imp___ZN7QString6appendERKS_)();
          piVar10 = piVar9 + -1;
          if (*local_3c == 0) {
LAB_0000b0d0:
            piVar9[1] = 4;
            *piVar9 = 2;
            piVar9[-1] = (int)local_3c;
            piVar9[-2] = 0xb0e9;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
          }
          else if (*local_3c != -1) {
            LOCK();
            *local_3c = *local_3c + -1;
            UNLOCK();
            if (*local_3c == 0) goto LAB_0000b0d0;
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
        }
        piVar9 = piVar10;
        if ((bVar5 & 4) != 0) {
          piVar10[2] = 4;
          piVar10[1] = 0x34;
          *piVar10 = (int)&local_38;
          piVar10[-1] = 0xae8e;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *piVar10 = (int)&local_38;
          piVar10[-1] = 0xae9a;
          (*___imp___ZN7QString6appendERKS_)();
          piVar9 = piVar10 + -1;
          if (*local_38 == 0) {
LAB_0000b0f0:
            piVar10[1] = 4;
            *piVar10 = 2;
            piVar10[-1] = (int)local_38;
            piVar10[-2] = 0xb109;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
          }
          else if (*local_38 != -1) {
            LOCK();
            *local_38 = *local_38 + -1;
            UNLOCK();
            if (*local_38 == 0) goto LAB_0000b0f0;
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
        }
        piVar10 = piVar9;
        if ((bVar5 & 8) != 0) {
          piVar9[2] = 4;
          piVar9[1] = 0x2f;
          *piVar9 = (int)&local_34;
          piVar9[-1] = 0xae1d;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *piVar9 = (int)&local_34;
          piVar9[-1] = 0xae29;
          (*___imp___ZN7QString6appendERKS_)();
          piVar10 = piVar9 + -1;
          if (*local_34 == 0) {
LAB_0000b110:
            piVar9[1] = 4;
            *piVar9 = 2;
            piVar9[-1] = (int)local_34;
            piVar9[-2] = 0xb129;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
          }
          else if (*local_34 != -1) {
            LOCK();
            *local_34 = *local_34 + -1;
            UNLOCK();
            if (*local_34 == 0) goto LAB_0000b110;
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
        }
        piVar9 = piVar10;
        if ((bVar5 & 0x10) != 0) {
          piVar10[2] = 0xb;
          piVar10[1] = 0x1ed;
          *piVar10 = (int)&local_30;
          piVar10[-1] = 0xadac;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *piVar10 = (int)&local_30;
          piVar10[-1] = 0xadb8;
          (*___imp___ZN7QString6appendERKS_)();
          piVar9 = piVar10 + -1;
          if (*local_30 == 0) {
LAB_0000b130:
            piVar10[1] = 4;
            *piVar10 = 2;
            piVar10[-1] = (int)local_30;
            piVar10[-2] = 0xb149;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
          }
          else if (*local_30 != -1) {
            LOCK();
            *local_30 = *local_30 + -1;
            UNLOCK();
            if (*local_30 == 0) goto LAB_0000b130;
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
        }
        piVar10 = piVar9;
        if ((bVar5 & 0x20) != 0) {
          piVar9[2] = 0xc;
          piVar9[1] = 0x1d5;
          *piVar9 = (int)&local_2c;
          piVar9[-1] = 0xad31;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *piVar9 = (int)&local_2c;
          piVar9[-1] = 0xad3d;
          (*___imp___ZN7QString6appendERKS_)();
          piVar10 = piVar9 + -1;
          if (*local_2c == 0) {
LAB_0000b092:
            piVar9[1] = 4;
            *piVar9 = 2;
            piVar9[-1] = (int)local_2c;
            piVar9[-2] = 0xb0ab;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
          }
          else if (*local_2c != -1) {
            LOCK();
            *local_2c = *local_2c + -1;
            UNLOCK();
            if (*local_2c == 0) goto LAB_0000b092;
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
        }
        if ((bVar5 & 0x40) != 0) {
          piVar10[2] = 10;
          piVar10[1] = 0x1e2;
          *piVar10 = (int)&local_28;
          piVar10[-1] = 0xac60;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *piVar10 = (int)&local_28;
          piVar10[-1] = 0xac6c;
          (*___imp___ZN7QString6appendERKS_)();
          if (*local_28 == 0) {
LAB_0000b0b0:
            piVar10[1] = 4;
            *piVar10 = 2;
            piVar10[-1] = (int)local_28;
            piVar10[-2] = 0xb0c9;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
          }
          else if (*local_28 != -1) {
            LOCK();
            *local_28 = *local_28 + -1;
            UNLOCK();
            if (*local_28 == 0) goto LAB_0000b0b0;
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          bVar5 = *(byte *)(uVar7 * 0x32 + 0x100 + uVar3 * 3000 + iVar1);
          piVar10 = piVar10 + -1;
        }
        ppuVar11 = (undefined **)piVar10;
        if ((char)bVar5 < '\0') {
          piVar10[2] = 10;
          piVar10[1] = 0x1f9;
          *piVar10 = (int)&local_24;
          piVar10[-1] = 0xacd4;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          *piVar10 = (int)&local_24;
          piVar10[-1] = 0xace0;
          (*___imp___ZN7QString6appendERKS_)();
          ppuVar11 = (undefined **)(piVar10 + -1);
          if (*local_24 == 0) {
LAB_0000b1b1:
            piVar10[-1] = (int)local_24;
            piVar10[1] = 4;
            *piVar10 = 2;
            piVar10[-2] = 0xb1ca;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            uVar3 = (uint)DAT_00002435;
            uVar7 = (uint)DAT_00002434;
          }
          else {
            if (*local_24 != -1) {
              LOCK();
              *local_24 = *local_24 + -1;
              UNLOCK();
              if (*local_24 == 0) goto LAB_0000b1b1;
            }
            uVar3 = (uint)DAT_00002435;
            uVar7 = (uint)DAT_00002434;
            ppuVar11 = (undefined **)(piVar10 + -1);
          }
        }
      }
      iVar1 = uVar7 * 0x32 + 0x100 + uVar3 * 3000;
      cVar4 = *(char *)(iVar1 + uVar12);
      local_50 = CONCAT31(local_50._1_3_,cVar4);
      if (cVar4 != '\0') {
        uVar6 = (uint)*(byte *)(local_54 + 0x4c);
        uVar8 = (uint)*(byte *)(local_54 + 0x4d);
        local_58 = uVar6;
        local_5c = uVar8;
        if (uVar6 < uVar8) {
          uVar6 = local_58;
          if (cVar4 != *(char *)(local_54 + 0x800 + uVar6)) {
            local_5c = uVar12;
            local_50 = local_54;
            do {
              uVar6 = uVar6 + 1;
              if (uVar6 == uVar8) {
                cVar4 = *(char *)(uVar7 * 0x32 + 0x103 + uVar3 * 3000);
                goto joined_r0x0000abe7;
              }
            } while (cVar4 != *(char *)(local_54 + 0x800 + uVar6));
          }
          local_58 = uVar6;
          ppuVar11[2] = (undefined *)0x1;
          ppuVar11[1] = &DAT_000025b4;
          *ppuVar11 = (undefined *)local_20;
          ppuVar11[-1] = (undefined *)0xaff8;
          (*___imp___ZN7QString15fromUtf8_helperEPKci)();
          pcVar2 = ___imp___ZN7QString6appendERKS_;
          *ppuVar11 = (undefined *)local_20;
          ppuVar11[-1] = (undefined *)0xb009;
          (*___imp___ZN7QString6appendERKS_)();
          if (*local_20[0] == 0) {
LAB_0000b150:
            ppuVar11[-1] = (undefined *)local_20[0];
            ppuVar11[1] = (undefined *)0x4;
            *ppuVar11 = (undefined *)0x2;
            ppuVar11[-2] = (undefined *)0xb169;
            (*___imp___ZN10QArrayData10deallocateEPS_jj)();
            iVar1 = *(int *)(local_54 + 0x2c);
          }
          else {
            if (*local_20[0] != -1) {
              LOCK();
              *local_20[0] = *local_20[0] + -1;
              UNLOCK();
              if (*local_20[0] == 0) goto LAB_0000b150;
            }
            iVar1 = *(int *)(local_54 + 0x2c);
          }
          if (iVar1 == 1) {
            ppuVar11[-1] = (undefined *)(local_54 + 0x494 + local_58 * 4);
            ppuVar11[-2] = (undefined *)0xb18b;
            (*pcVar2)();
          }
          else {
            ppuVar11[-1] = (undefined *)(local_54 + 0x128 + local_58 * 4);
            ppuVar11[-2] = (undefined *)0xb04d;
            (*pcVar2)();
          }
          uVar3 = (uint)DAT_00002435;
          uVar7 = (uint)DAT_00002434;
          ppuVar11 = ppuVar11 + -2;
          cVar4 = *(char *)(uVar7 * 0x32 + 0x103 + uVar3 * 3000);
          goto joined_r0x0000abe7;
        }
      }
      cVar4 = *(char *)(iVar1 + 3);
    }
joined_r0x0000abe7:
    uVar12 = uVar12 + 2;
  } while (uVar12 != 0x2f);
  if (cVar4 == '\x02') {
    cVar4 = *(char *)(uVar7 * 0x32 + 0x10a + uVar3 * 3000);
    if (cVar4 != '\0') {
      uVar3 = (uint)*(byte *)(local_54 + 0x46);
      if (uVar3 <= *(byte *)(local_54 + 0x47)) {
        if (cVar4 != *(char *)(local_54 + 0x800 + uVar3)) {
          do {
            uVar3 = uVar3 + 1;
            if ((int)(uint)*(byte *)(local_54 + 0x47) < (int)uVar3) goto LAB_0000ac02;
          } while (cVar4 != *(char *)(local_54 + 0x800 + uVar3));
        }
        *ppuVar11 = &DAT_000025b4;
        ppuVar11[-1] = (undefined *)0xb371;
        QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
        if (*(int *)(local_54 + 0x2c) == 1) {
          *ppuVar11 = (undefined *)(local_54 + 0x494 + uVar3 * 4);
          ppuVar11[-1] = (undefined *)0xb3ae;
          (*___imp___ZN7QString6appendERKS_)();
        }
        else {
          *ppuVar11 = (undefined *)(local_54 + 0x128 + uVar3 * 4);
          ppuVar11[-1] = (undefined *)0xb393;
          (*___imp___ZN7QString6appendERKS_)();
        }
        goto LAB_0000b393;
      }
    }
  }
  else if (cVar4 == '\x03') {
    iVar1 = uVar7 * 0x32 + 0x100 + uVar3 * 3000;
    cVar4 = *(char *)(iVar1 + 0xb);
    if (cVar4 == '\x01') {
      if ((*(char *)(iVar1 + 10) == '\0') && (*(char *)(iVar1 + 0xe) == '\0')) {
        *ppuVar11 = &DAT_000025b4;
        ppuVar11[-1] = (undefined *)0xb333;
        QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
        if (*(int *)(local_54 + 0x2c) == 1) {
          *ppuVar11 = (undefined *)(local_54 + 0x494 + (uint)*(byte *)(local_54 + 0x4a) * 4);
          ppuVar11[-1] = (undefined *)0xb659;
          (*___imp___ZN7QString6appendERKS_)();
        }
        else {
          *ppuVar11 = (undefined *)(local_54 + 0x128 + (uint)*(byte *)(local_54 + 0x4a) * 4);
          ppuVar11[-1] = (undefined *)0xb35d;
          (*___imp___ZN7QString6appendERKS_)();
        }
        goto LAB_0000b393;
      }
    }
    else if (cVar4 == '\x04') {
      if ((*(char *)(iVar1 + 10) == '\0') && (*(char *)(iVar1 + 0xe) == '\0')) {
        *ppuVar11 = &DAT_000025b4;
        ppuVar11[-1] = (undefined *)0xb2e0;
        QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
        if (*(int *)(local_54 + 0x2c) == 1) {
          *ppuVar11 = (undefined *)(local_54 + 0x498 + (uint)*(byte *)(local_54 + 0x4a) * 4);
          ppuVar11[-1] = (undefined *)0xb671;
          (*___imp___ZN7QString6appendERKS_)();
        }
        else {
          *ppuVar11 = (undefined *)(local_54 + 300 + (uint)*(byte *)(local_54 + 0x4a) * 4);
          ppuVar11[-1] = (undefined *)0xb30a;
          (*___imp___ZN7QString6appendERKS_)();
        }
        goto LAB_0000b393;
      }
    }
    else if (cVar4 == '\x02') {
      if ((*(char *)(iVar1 + 10) == '\0') && (*(char *)(iVar1 + 0xe) == '\0')) {
        *ppuVar11 = &DAT_000025b4;
        ppuVar11[-1] = (undefined *)0xb232;
        QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
        if (*(int *)(local_54 + 0x2c) == 1) {
          *ppuVar11 = (undefined *)(local_54 + 0x49c + (uint)*(byte *)(local_54 + 0x4a) * 4);
          ppuVar11[-1] = (undefined *)0xb641;
          (*___imp___ZN7QString6appendERKS_)();
        }
        else {
          *ppuVar11 = (undefined *)(local_54 + 0x130 + (uint)*(byte *)(local_54 + 0x4a) * 4);
          ppuVar11[-1] = (undefined *)0xb25c;
          (*___imp___ZN7QString6appendERKS_)();
        }
        goto LAB_0000b393;
      }
    }
    else if (cVar4 == '\0') {
      if (*(char *)(iVar1 + 0xe) == '\x01') {
        if (*(char *)(iVar1 + 10) == '\0') {
          *ppuVar11 = &DAT_000025b4;
          ppuVar11[-1] = (undefined *)0xb52d;
          QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
          if (*(int *)(local_54 + 0x2c) == 1) {
            *ppuVar11 = (undefined *)(local_54 + 0x4a0 + (uint)*(byte *)(local_54 + 0x4a) * 4);
            ppuVar11[-1] = (undefined *)0xb629;
            (*___imp___ZN7QString6appendERKS_)();
          }
          else {
            *ppuVar11 = (undefined *)(local_54 + 0x134 + (uint)*(byte *)(local_54 + 0x4a) * 4);
            ppuVar11[-1] = (undefined *)0xb557;
            (*___imp___ZN7QString6appendERKS_)();
          }
        }
        else {
          cVar4 = *(char *)(iVar1 + 10);
          if (cVar4 == '\x01') {
            *ppuVar11 = &DAT_000025b4;
            ppuVar11[-1] = (undefined *)0xb5e7;
            QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
            if (*(int *)(local_54 + 0x2c) == 1) {
              *ppuVar11 = (undefined *)(local_54 + 0x4a8 + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb719;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *ppuVar11 = (undefined *)(local_54 + 0x13c + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb611;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else if (cVar4 == '\x02') {
            *ppuVar11 = &DAT_000025b4;
            ppuVar11[-1] = (undefined *)0xb5a9;
            QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
            if (*(int *)(local_54 + 0x2c) == 1) {
              *ppuVar11 = (undefined *)(local_54 + 0x4b0 + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb6a1;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *ppuVar11 = (undefined *)(local_54 + 0x144 + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb5d3;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else {
            if (cVar4 != '\x04') goto LAB_0000ac02;
            *ppuVar11 = &DAT_000025b4;
            ppuVar11[-1] = (undefined *)0xb452;
            QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
            if (*(int *)(local_54 + 0x2c) == 1) {
              *ppuVar11 = (undefined *)(local_54 + 0x4b8 + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb6b9;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *ppuVar11 = (undefined *)(local_54 + 0x14c + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb47c;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
        }
      }
      else {
        if (*(char *)(iVar1 + 0xe) != -1) goto LAB_0000ac02;
        if (*(char *)(iVar1 + 10) == '\0') {
          *ppuVar11 = &DAT_000025b4;
          ppuVar11[-1] = (undefined *)0xb4ef;
          QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
          if (*(int *)(local_54 + 0x2c) == 1) {
            *ppuVar11 = (undefined *)(local_54 + 0x4a4 + (uint)*(byte *)(local_54 + 0x4a) * 4);
            ppuVar11[-1] = (undefined *)0xb689;
            (*___imp___ZN7QString6appendERKS_)();
          }
          else {
            *ppuVar11 = (undefined *)(local_54 + 0x138 + (uint)*(byte *)(local_54 + 0x4a) * 4);
            ppuVar11[-1] = (undefined *)0xb519;
            (*___imp___ZN7QString6appendERKS_)();
          }
        }
        else if (*(char *)(iVar1 + 10) == '\x01') {
          *ppuVar11 = &DAT_000025b4;
          ppuVar11[-1] = (undefined *)0xb3ee;
          QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
          if (*(int *)(local_54 + 0x2c) == 1) {
            *ppuVar11 = (undefined *)(local_54 + 0x4ac + (uint)*(byte *)(local_54 + 0x4a) * 4);
            ppuVar11[-1] = (undefined *)0xb6e9;
            (*___imp___ZN7QString6appendERKS_)();
          }
          else {
            *ppuVar11 = (undefined *)(local_54 + 0x140 + (uint)*(byte *)(local_54 + 0x4a) * 4);
            ppuVar11[-1] = (undefined *)0xb418;
            (*___imp___ZN7QString6appendERKS_)();
          }
        }
        else {
          cVar4 = *(char *)(uVar7 * 0x32 + 0x10a + uVar3 * 3000);
          if (cVar4 == '\x02') {
            *ppuVar11 = &DAT_000025b4;
            ppuVar11[-1] = (undefined *)0xb56b;
            QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
            if (*(int *)(local_54 + 0x2c) == 1) {
              *ppuVar11 = (undefined *)(local_54 + 0x4b4 + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb6d1;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *ppuVar11 = (undefined *)(local_54 + 0x148 + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb595;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
          else {
            if (cVar4 != '\x04') goto LAB_0000ac02;
            *ppuVar11 = &DAT_000025b4;
            ppuVar11[-1] = (undefined *)0xb4b1;
            QString::operator+=((QString *)*ppuVar11,ppuVar11[1]);
            if (*(int *)(local_54 + 0x2c) == 1) {
              *ppuVar11 = (undefined *)(local_54 + 0x4bc + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb701;
              (*___imp___ZN7QString6appendERKS_)();
            }
            else {
              *ppuVar11 = (undefined *)(local_54 + 0x150 + (uint)*(byte *)(local_54 + 0x4a) * 4);
              ppuVar11[-1] = (undefined *)0xb4db;
              (*___imp___ZN7QString6appendERKS_)();
            }
          }
        }
      }
LAB_0000b393:
      ppuVar11 = ppuVar11 + -1;
    }
  }
LAB_0000ac02:
  *ppuVar11 = (undefined *)&local_44;
  ppuVar11[-1] = (undefined *)0xac1a;
  (*___imp___ZN9QTextEdit7setTextERK7QString)();
  piVar9 = local_44;
  if (*local_44 == 0) {
LAB_0000b190:
    ppuVar11[1] = (undefined *)0x4;
    *ppuVar11 = (undefined *)0x2;
    ppuVar11[-1] = (undefined *)piVar9;
    ppuVar11[-2] = (undefined *)0xb1a9;
    (*___imp___ZN10QArrayData10deallocateEPS_jj)();
    return;
  }
  if (*local_44 != -1) {
    LOCK();
    *local_44 = *local_44 + -1;
    UNLOCK();
    if (*local_44 == 0) goto LAB_0000b190;
  }
  return;
}

