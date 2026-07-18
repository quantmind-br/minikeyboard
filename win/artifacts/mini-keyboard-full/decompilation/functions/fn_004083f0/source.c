
void __thiscall FUN_004083f0(int param_1,uint param_2)

{
  int iVar1;
  QArrayData *pQVar2;
  QArrayData *pQVar3;
  char cVar4;
  byte bVar5;
  uint uVar6;
  byte *pbVar7;
  char *pcVar8;
  char *in_stack_ffffff78;
  int in_stack_ffffff7c;
  QString *pQVar9;
  code *pcVar10;
  char cStack_5d;
  QArrayData *local_44;
  QArrayData *local_40;
  int *local_3c;
  int *local_38;
  int *local_34;
  int *local_30;
  int *local_2c;
  int *local_28;
  int *local_24;
  QArrayData *local_20 [4];
  
  pcVar10 = fromAscii_helper_exref;
  local_44 = (QArrayData *)QString::fromAscii_helper("",0);
  cStack_5d = '\x02';
  pcVar8 = &DAT_0061e175 + (param_2 & 0xff) * 3000;
  do {
    pbVar7 = (byte *)(pcVar8 + 7);
    cVar4 = *pcVar8;
LAB_00408453:
    do {
      if (cVar4 == '\x01') {
        bVar5 = *pbVar7;
        if (bVar5 != 0) {
          if ((bVar5 & 1) != 0) {
            QString::fromUtf8_helper((char *)&local_40,0x49f658);
            QString::append((QString *)&local_40);
            if (*(int *)local_40 == 0) {
LAB_00408b70:
              pcVar10 = (code *)0x4;
              QArrayData::deallocate(local_40,2,4);
            }
            else if (*(int *)local_40 != -1) {
              LOCK();
              *(int *)local_40 = *(int *)local_40 + -1;
              UNLOCK();
              if (*(int *)local_40 == 0) goto LAB_00408b70;
            }
            bVar5 = *pbVar7;
          }
          if ((bVar5 & 2) != 0) {
            QString::fromUtf8_helper(in_stack_ffffff78,in_stack_ffffff7c);
            pQVar9 = (QString *)&local_3c;
            in_stack_ffffff7c = 0x408750;
            QString::append(pQVar9);
            if (*local_3c == 0) {
LAB_00408bd0:
              in_stack_ffffff7c = 0x408be9;
              QArrayData::deallocate((QArrayData *)pQVar9,2,4);
            }
            else if (*local_3c != -1) {
              LOCK();
              *local_3c = *local_3c + -1;
              UNLOCK();
              if (*local_3c == 0) goto LAB_00408bd0;
            }
            bVar5 = *pbVar7;
          }
          if ((bVar5 & 4) != 0) {
            QString::fromUtf8_helper(in_stack_ffffff78,in_stack_ffffff7c);
            pQVar9 = (QString *)&local_38;
            in_stack_ffffff7c = 0x4086fb;
            QString::append(pQVar9);
            if (*local_38 == 0) {
LAB_00408bf0:
              in_stack_ffffff7c = 0x408c09;
              QArrayData::deallocate((QArrayData *)pQVar9,2,4);
            }
            else if (*local_38 != -1) {
              LOCK();
              *local_38 = *local_38 + -1;
              UNLOCK();
              if (*local_38 == 0) goto LAB_00408bf0;
            }
            bVar5 = *pbVar7;
          }
          if ((bVar5 & 8) != 0) {
            QString::fromUtf8_helper(in_stack_ffffff78,in_stack_ffffff7c);
            pQVar9 = (QString *)&local_34;
            in_stack_ffffff7c = 0x4086a0;
            QString::append(pQVar9);
            if (*local_34 == 0) {
LAB_00408c10:
              in_stack_ffffff7c = 0x408c29;
              QArrayData::deallocate((QArrayData *)pQVar9,2,4);
            }
            else if (*local_34 != -1) {
              LOCK();
              *local_34 = *local_34 + -1;
              UNLOCK();
              if (*local_34 == 0) goto LAB_00408c10;
            }
            bVar5 = *pbVar7;
          }
          if ((bVar5 & 0x10) != 0) {
            QString::fromUtf8_helper(in_stack_ffffff78,in_stack_ffffff7c);
            pQVar9 = (QString *)&local_30;
            in_stack_ffffff7c = 0x40864b;
            QString::append(pQVar9);
            if (*local_30 == 0) {
LAB_00408c30:
              in_stack_ffffff7c = 0x408c49;
              QArrayData::deallocate((QArrayData *)pQVar9,2,4);
            }
            else if (*local_30 != -1) {
              LOCK();
              *local_30 = *local_30 + -1;
              UNLOCK();
              if (*local_30 == 0) goto LAB_00408c30;
            }
            bVar5 = *pbVar7;
          }
          if ((bVar5 & 0x20) != 0) {
            QString::fromUtf8_helper(in_stack_ffffff78,in_stack_ffffff7c);
            pQVar9 = (QString *)&local_2c;
            in_stack_ffffff7c = 0x4085ee;
            QString::append(pQVar9);
            if (*local_2c == 0) {
LAB_00408b90:
              in_stack_ffffff7c = 0x408ba9;
              QArrayData::deallocate((QArrayData *)pQVar9,2,4);
            }
            else if (*local_2c != -1) {
              LOCK();
              *local_2c = *local_2c + -1;
              UNLOCK();
              if (*local_2c == 0) goto LAB_00408b90;
            }
            bVar5 = *pbVar7;
          }
          if ((bVar5 & 0x40) != 0) {
            QString::fromUtf8_helper(in_stack_ffffff78,in_stack_ffffff7c);
            pQVar9 = (QString *)&local_28;
            in_stack_ffffff7c = 0x40852b;
            QString::append(pQVar9);
            if (*local_28 == 0) {
LAB_00408bb0:
              in_stack_ffffff7c = 0x408bc9;
              QArrayData::deallocate((QArrayData *)pQVar9,2,4);
            }
            else if (*local_28 != -1) {
              LOCK();
              *local_28 = *local_28 + -1;
              UNLOCK();
              if (*local_28 == 0) goto LAB_00408bb0;
            }
            bVar5 = *pbVar7;
          }
          if ((char)bVar5 < '\0') {
            QString::fromUtf8_helper((char *)&local_24,0x49f693);
            pQVar9 = (QString *)&local_24;
            QString::append(pQVar9);
            if (*local_24 == 0) {
LAB_004085a5:
              pcVar10 = (code *)0x4;
              QArrayData::deallocate((QArrayData *)pQVar9,2,4);
            }
            else if (*local_24 != -1) {
              LOCK();
              *local_24 = *local_24 + -1;
              UNLOCK();
              if (*local_24 == 0) goto LAB_004085a5;
            }
          }
        }
        if (pbVar7[1] != 0) {
          iVar1 = *(int *)(param_1 + 0x20);
          uVar6 = (uint)*(byte *)(iVar1 + 0x4c);
          if (uVar6 < *(byte *)(iVar1 + 0x4d)) {
            bVar5 = *(byte *)(iVar1 + 0x800 + uVar6);
            while (pbVar7[1] != bVar5) {
              uVar6 = uVar6 + 1;
              if (uVar6 == *(byte *)(iVar1 + 0x4d)) goto LAB_00408445;
              bVar5 = *(byte *)(iVar1 + 0x800 + uVar6);
            }
            QString::append((QString *)(iVar1 + 0x128 + uVar6 * 4));
            pbVar7 = pbVar7 + 2;
            cVar4 = *pcVar8;
            if ((byte *)(pcVar8 + 0x2b) == pbVar7) break;
            goto LAB_00408453;
          }
        }
LAB_00408445:
        cVar4 = *pcVar8;
      }
      pbVar7 = pbVar7 + 2;
    } while ((byte *)(pcVar8 + 0x2b) != pbVar7);
    if (cVar4 == '\x02') {
      if (pcVar8[7] != '\0') {
        iVar1 = *(int *)(param_1 + 0x20);
        uVar6 = (uint)*(byte *)(iVar1 + 0x46);
        if (uVar6 <= *(byte *)(iVar1 + 0x47)) {
          do {
            if (pcVar8[7] == *(char *)(iVar1 + 0x800 + uVar6)) {
              QString::append((QString *)(iVar1 + 0x128 + uVar6 * 4));
              break;
            }
            uVar6 = uVar6 + 1;
          } while ((int)uVar6 <= (int)(uint)*(byte *)(iVar1 + 0x47));
        }
      }
    }
    else if (cVar4 == '\x03') {
      cVar4 = pcVar8[8];
      if (cVar4 == '\x01') {
        if ((pcVar8[0xb] == '\0') && (pcVar8[7] == '\0')) {
          QString::append((QString *)
                          (*(int *)(param_1 + 0x20) + 0x128 +
                          (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
        }
      }
      else if (cVar4 == '\x04') {
        if ((pcVar8[0xb] == '\0') && (pcVar8[7] == '\0')) {
          QString::append((QString *)
                          (*(int *)(param_1 + 0x20) + 300 +
                          (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
        }
      }
      else if (cVar4 == '\x02') {
        if ((pcVar8[0xb] == '\0') && (pcVar8[7] == '\0')) {
          QString::append((QString *)
                          (*(int *)(param_1 + 0x20) + 0x130 +
                          (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
        }
      }
      else if (cVar4 == '\0') {
        if (pcVar8[0xb] == '\x01') {
          if (pcVar8[7] == '\0') {
            QString::append((QString *)
                            (*(int *)(param_1 + 0x20) + 0x134 +
                            (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
          }
          else {
            cVar4 = pcVar8[7];
            if (cVar4 == '\x01') {
              QString::append((QString *)
                              (*(int *)(param_1 + 0x20) + 0x13c +
                              (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
            }
            else if (cVar4 == '\x02') {
              QString::append((QString *)
                              (*(int *)(param_1 + 0x20) + 0x144 +
                              (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
            }
            else if (cVar4 == '\x04') {
              QString::append((QString *)
                              (*(int *)(param_1 + 0x20) + 0x14c +
                              (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
            }
          }
        }
        else if (pcVar8[0xb] == -1) {
          cVar4 = pcVar8[7];
          if (cVar4 == '\0') {
            QString::append((QString *)
                            (*(int *)(param_1 + 0x20) + 0x138 +
                            (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
          }
          else if (cVar4 == '\x01') {
            QString::append((QString *)
                            (*(int *)(param_1 + 0x20) + 0x140 +
                            (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
          }
          else if (cVar4 == '\x02') {
            QString::append((QString *)
                            (*(int *)(param_1 + 0x20) + 0x148 +
                            (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
          }
          else if (cVar4 == '\x04') {
            QString::append((QString *)
                            (*(int *)(param_1 + 0x20) + 0x150 +
                            (uint)*(byte *)(*(int *)(param_1 + 0x20) + 0x4a) * 4));
          }
        }
      }
    }
    switch(cStack_5d + -1) {
    case '\x01':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x02':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x03':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x04':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x05':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x06':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\a':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\b':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\t':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\n':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\v':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\f':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\r':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x0e':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x0f':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x10':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x11':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x12':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x13':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x14':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x15':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x16':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x17':
      QAbstractButton::setText((QString *)&local_44);
      break;
    case '\x18':
      QAbstractButton::setText((QString *)&local_44);
    }
    pQVar9 = (QString *)local_20;
    QString::fromUtf8_helper((char *)pQVar9,0x49f38e);
    pQVar3 = local_20[0];
    pQVar2 = local_44;
    local_20[0] = local_44;
    local_44 = pQVar3;
    if (*(int *)pQVar2 == 0) {
LAB_00408c50:
      pcVar10 = (code *)0x4;
      QArrayData::deallocate(pQVar2,2,4);
    }
    else if (*(int *)pQVar2 != -1) {
      LOCK();
      *(int *)pQVar2 = *(int *)pQVar2 + -1;
      UNLOCK();
      if (*(int *)pQVar2 == 0) goto LAB_00408c50;
    }
    if (((pcVar8[8] == '\0') && (pcVar8[0xb] == '\0')) && (pcVar8[7] == '\0')) {
      switch(cStack_5d + -1) {
      default:
        goto switchD_004088d2_caseD_0;
      case '\x01':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4093d7;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x02':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4093a7;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x03':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x409377;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x04':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x409347;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x05':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x409317;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x06':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4092e7;
        QAbstractButton::setText(pQVar9);
        break;
      case '\a':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4092b7;
        QAbstractButton::setText(pQVar9);
        break;
      case '\b':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x409284;
        QAbstractButton::setText(pQVar9);
        break;
      case '\t':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x409251;
        QAbstractButton::setText(pQVar9);
        break;
      case '\n':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x40921e;
        QAbstractButton::setText(pQVar9);
        break;
      case '\v':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4091eb;
        QAbstractButton::setText(pQVar9);
        break;
      case '\f':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4091b8;
        QAbstractButton::setText(pQVar9);
        break;
      case '\r':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x409185;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x0e':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x409152;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x0f':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x40911f;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x10':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4090ec;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x11':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x4090bc;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x12':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x40908c;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x13':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x40905c;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x14':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x40902c;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x15':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x408ffc;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x16':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x408fcc;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x17':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x408f9f;
        QAbstractButton::setText(pQVar9);
        break;
      case '\x18':
        local_20[0] = (QArrayData *)(*pcVar10)(&DAT_0049f367,4);
        in_stack_ffffff7c = 0x408f34;
        QAbstractButton::setText(pQVar9);
      }
      if (*(int *)local_20[0] == 0) {
LAB_00408f56:
        in_stack_ffffff7c = 0x408f6f;
        QArrayData::deallocate((QArrayData *)pQVar9,2,4);
      }
      else if (*(int *)local_20[0] != -1) {
        LOCK();
        *(int *)local_20[0] = *(int *)local_20[0] + -1;
        UNLOCK();
        if (*(int *)local_20[0] == 0) goto LAB_00408f56;
      }
    }
    else {
switchD_004088d2_caseD_0:
      if (cStack_5d == '<') break;
    }
    pcVar8 = pcVar8 + 0x32;
    cStack_5d = cStack_5d + '\x01';
  } while( true );
  if (*(int *)local_44 == 0) {
LAB_00408ea2:
    QArrayData::deallocate(local_44,2,4);
    return;
  }
  if (*(int *)local_44 != -1) {
    LOCK();
    *(int *)local_44 = *(int *)local_44 + -1;
    UNLOCK();
    if (*(int *)local_44 == 0) goto LAB_00408ea2;
  }
  return;
}

