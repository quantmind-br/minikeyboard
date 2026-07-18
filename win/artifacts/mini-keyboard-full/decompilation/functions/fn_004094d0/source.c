
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 FUN_004094d0(undefined4 param_1,char **param_2)

{
  QArrayData *pQVar1;
  undefined4 uVar2;
  QArrayData *pQVar3;
  QArrayData *pQVar4;
  int iVar5;
  code *pcVar6;
  QString *pQVar7;
  QArrayData *pQVar8;
  
  QApplication::QApplication((QApplication *)&stack0xfffff650,&param_1,param_2,0x50e02);
  _DAT_0061e060 = &stack0xfffff650;
  QTranslator::QTranslator((QTranslator *)&stack0xfffff658,(QObject *)0x0);
  pcVar6 = fromAscii_helper_exref;
  pQVar8 = (QArrayData *)shared_null_exref;
  pQVar3 = (QArrayData *)shared_null_exref;
  pQVar4 = (QArrayData *)shared_null_exref;
  pQVar1 = (QArrayData *)QString::fromAscii_helper(":/lanague_cn.qm",0xf);
  pQVar7 = (QString *)&stack0xfffff64c;
  QTranslator::load((QString *)&stack0xfffff648,pQVar7,(QString *)&stack0xfffff660,
                    (QString *)&stack0xfffff668);
  if (*(int *)pQVar1 == 0) {
LAB_00409762:
    QArrayData::deallocate(pQVar1,2,4);
    iVar5 = *(int *)pQVar8;
    if (iVar5 != 0) goto LAB_004095c9;
LAB_0040978b:
    pQVar7 = (QString *)0x4;
    pcVar6 = (code *)0x2;
    QArrayData::deallocate(pQVar8,2,4);
    iVar5 = *(int *)pQVar3;
    if (iVar5 == 0) goto LAB_004097b4;
LAB_004095ee:
    if (iVar5 != -1) {
      LOCK();
      *(int *)pQVar3 = *(int *)pQVar3 + -1;
      UNLOCK();
      if (*(int *)pQVar3 == 0) goto LAB_004097b4;
    }
    iVar5 = *(int *)pQVar4;
    if (iVar5 != 0) goto LAB_00409613;
LAB_004097dd:
    QArrayData::deallocate(pQVar4,2,4);
  }
  else {
    if (*(int *)pQVar1 != -1) {
      LOCK();
      *(int *)pQVar1 = *(int *)pQVar1 + -1;
      UNLOCK();
      if (*(int *)pQVar1 == 0) goto LAB_00409762;
    }
    iVar5 = *(int *)pQVar8;
    if (iVar5 == 0) goto LAB_0040978b;
LAB_004095c9:
    if (iVar5 != -1) {
      LOCK();
      *(int *)pQVar8 = *(int *)pQVar8 + -1;
      UNLOCK();
      if (*(int *)pQVar8 == 0) goto LAB_0040978b;
    }
    iVar5 = *(int *)pQVar3;
    if (iVar5 != 0) goto LAB_004095ee;
LAB_004097b4:
    pQVar7 = (QString *)0x4097cd;
    QArrayData::deallocate(pQVar3,2,4);
    iVar5 = *(int *)pQVar4;
    if (iVar5 == 0) goto LAB_004097dd;
LAB_00409613:
    if (iVar5 != -1) {
      LOCK();
      *(int *)pQVar4 = *(int *)pQVar4 + -1;
      UNLOCK();
      if (*(int *)pQVar4 == 0) goto LAB_004097dd;
    }
  }
  QCoreApplication::installTranslator((QTranslator *)&stack0xfffff658);
  pQVar8 = (QArrayData *)0xffffffff;
  QFont::QFont((QFont *)&stack0xfffff660,(QString *)&stack0xfffff668,-1,-1,false);
  if (*(int *)pQVar8 == 0) {
LAB_00409820:
    QArrayData::deallocate(pQVar8,2,4);
  }
  else if (*(int *)pQVar8 != -1) {
    LOCK();
    *(int *)pQVar8 = *(int *)pQVar8 + -1;
    UNLOCK();
    if (*(int *)pQVar8 == 0) goto LAB_00409820;
  }
  QFont::setPixelSize(0xf);
  QApplication::setFont((QFont *)&stack0xfffff660,(char *)0x0);
  FUN_0045f740();
  if (DAT_00620476 == '\x02') {
    pQVar8 = (QArrayData *)(*pcVar6)();
    QWidget::setWindowTitle(pQVar7);
  }
  else {
    pQVar8 = (QArrayData *)(*pcVar6)();
    QWidget::setWindowTitle(pQVar7);
  }
  if (*(int *)pQVar8 != 0) {
    if (*(int *)pQVar8 == -1) goto LAB_0040971d;
    LOCK();
    *(int *)pQVar8 = *(int *)pQVar8 + -1;
    UNLOCK();
    if (*(int *)pQVar8 != 0) goto LAB_0040971d;
  }
  QArrayData::deallocate(pQVar8,2,4);
LAB_0040971d:
  QWidget::show();
  uVar2 = QApplication::exec();
  FUN_00411b70();
  QFont::~QFont((QFont *)&stack0xfffff660);
  QTranslator::~QTranslator((QTranslator *)&stack0xfffff658);
  QApplication::~QApplication((QApplication *)&stack0xfffff650);
  return uVar2;
}

