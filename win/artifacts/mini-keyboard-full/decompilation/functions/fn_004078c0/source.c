
void __fastcall FUN_004078c0(int param_1)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  QArrayData *pQVar3;
  QString *pQVar4;
  uint uVar5;
  code *pcVar6;
  
  uVar5 = (uint)DAT_00620475;
  puVar1 = &DAT_0061e176 + uVar5 * 3000;
  do {
    *puVar1 = 0;
    puVar1[0x32] = 0;
    puVar2 = puVar1 + 1;
    puVar1[100] = 0;
    puVar1[0x96] = 0;
    puVar1[200] = 0;
    puVar1[0xfa] = 0;
    puVar1[300] = 0;
    puVar1[0x15e] = 0;
    puVar1[400] = 0;
    puVar1[0x1c2] = 0;
    puVar1[500] = 0;
    puVar1[0x226] = 0;
    puVar1[600] = 0;
    puVar1[0x28a] = 0;
    puVar1[700] = 0;
    puVar1[0x2ee] = 0;
    puVar1[800] = 0;
    puVar1[0x352] = 0;
    puVar1[900] = 0;
    puVar1[0x3b6] = 0;
    puVar1[1000] = 0;
    puVar1[0x41a] = 0;
    puVar1[0x44c] = 0;
    puVar1[0x47e] = 0;
    puVar1 = puVar2;
  } while (puVar2 != &DAT_0061e1a4 + uVar5 * 3000);
  pcVar6 = fromAscii_helper_exref;
  pQVar3 = (QArrayData *)QString::fromAscii_helper("NULL",4);
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_004080b0:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_004080b0;
  }
  pQVar3 = (QArrayData *)(*pcVar6)(&DAT_0049f367,*(undefined4 *)(*(int *)(param_1 + 0x1c) + 0x74));
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_004080d0:
    pcVar6 = (code *)0x4;
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_004080d0;
  }
  pQVar3 = (QArrayData *)(*pcVar6)(&DAT_0049f367,4);
  pcVar6 = (code *)0x407a53;
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408110:
    pcVar6 = (code *)0x408129;
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408110;
  }
  pQVar4 = (QString *)(*pcVar6)();
  QAbstractButton::setText(pQVar4);
  if (*(int *)pQVar4 == 0) {
LAB_004080f0:
    QArrayData::deallocate((QArrayData *)pQVar4,2,4);
  }
  else if (*(int *)pQVar4 != -1) {
    LOCK();
    *(int *)pQVar4 = *(int *)pQVar4 + -1;
    UNLOCK();
    if (*(int *)pQVar4 == 0) goto LAB_004080f0;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408190:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408190;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408170:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408170;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408150:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408150;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408130:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408130;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408390:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408390;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408370:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408370;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408350:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408350;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408330:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408330;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408310:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408310;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_004082f0:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_004082f0;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_004082d0:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_004082d0;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_004082b0:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_004082b0;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408290:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408290;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408270:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408270;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408250:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408250;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408230:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408230;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_00408210:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_00408210;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_004081f0:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_004081f0;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 == 0) {
LAB_004081d1:
    QArrayData::deallocate(pQVar3,2,4);
  }
  else if (*(int *)pQVar3 != -1) {
    LOCK();
    *(int *)pQVar3 = *(int *)pQVar3 + -1;
    UNLOCK();
    if (*(int *)pQVar3 == 0) goto LAB_004081d1;
  }
  pQVar3 = (QArrayData *)(*pcVar6)();
  QAbstractButton::setText((QString *)&stack0xffffffe0);
  if (*(int *)pQVar3 != 0) {
    if (*(int *)pQVar3 != -1) {
      LOCK();
      *(int *)pQVar3 = *(int *)pQVar3 + -1;
      UNLOCK();
      if (*(int *)pQVar3 == 0) goto LAB_004081b0;
    }
    return;
  }
LAB_004081b0:
  QArrayData::deallocate(pQVar3,2,4);
  return;
}

