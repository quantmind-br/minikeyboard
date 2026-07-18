
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::on_change_clicked() */

void Widget::on_change_clicked(void)

{
  int iVar1;
  QWidget *in_ECX;
  undefined4 **ppuVar2;
  QString *pQVar3;
  QString *in_stack_ffffff90;
  undefined4 *local_60;
  undefined4 *local_5c;
  undefined4 *local_58;
  undefined4 *local_54;
  code *local_48;
  undefined4 *local_44;
  undefined4 *local_40;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28 [6];
  
  local_5c = (undefined4 *)0x0;
  local_60 = (undefined4 *)0x5db0b;
  (*___imp___ZN11QTranslatorC1EP7QObject)();
  ppuVar2 = &local_60;
  iVar1 = *(int *)(in_ECX + 0x2c);
  if (DAT_00002436 == '\x02') {
    if (iVar1 == 0) {
      local_5c = (undefined4 *)0x10;
      local_60 = (undefined4 *)&DAT_00002d64;
      local_28[0] = ___imp___ZN10QArrayData11shared_nullE;
      local_2c = ___imp___ZN10QArrayData11shared_nullE;
      local_30 = ___imp___ZN10QArrayData11shared_nullE;
      local_48 = ___imp___ZN7QString16fromAscii_helperEPKci;
      local_34 = (*___imp___ZN7QString16fromAscii_helperEPKci)();
      local_60 = &local_34;
      local_54 = local_28;
      local_58 = &local_2c;
      local_5c = &local_30;
      local_44 = local_60;
      local_40 = local_58;
      (*___imp___ZN11QTranslator4loadERK7QStringS2_S2_S2_)();
      QString::~QString(in_stack_ffffff90);
      QString::~QString(in_stack_ffffff90);
      QString::~QString(in_stack_ffffff90);
      QString::~QString(in_stack_ffffff90);
      (*___imp___ZN16QCoreApplication17installTranslatorEP11QTranslator)();
      *(undefined4 *)(in_ECX + 0x2c) = 1;
      Ui_Widget::retranslateUi(in_ECX);
      local_28[0] = (*local_48)();
      pQVar3 = (QString *)0x5ddbe;
      (*___imp___ZN15QAbstractButton7setTextERK7QString)();
      goto LAB_0005dbfe;
    }
    if (iVar1 != 1) goto LAB_0005db2b;
    local_5c = (undefined4 *)0x10;
    local_60 = (undefined4 *)&DAT_00002d7c;
    local_28[0] = ___imp___ZN10QArrayData11shared_nullE;
    local_2c = ___imp___ZN10QArrayData11shared_nullE;
    local_30 = ___imp___ZN10QArrayData11shared_nullE;
    local_48 = ___imp___ZN7QString16fromAscii_helperEPKci;
    local_34 = (*___imp___ZN7QString16fromAscii_helperEPKci)();
    local_60 = &local_34;
    local_54 = local_28;
    local_58 = &local_2c;
    local_5c = &local_30;
    local_44 = local_60;
    local_40 = local_58;
    (*___imp___ZN11QTranslator4loadERK7QStringS2_S2_S2_)();
    QString::~QString(in_stack_ffffff90);
    QString::~QString(in_stack_ffffff90);
    QString::~QString(in_stack_ffffff90);
    QString::~QString(in_stack_ffffff90);
    (*___imp___ZN16QCoreApplication17installTranslatorEP11QTranslator)();
    *(undefined4 *)(in_ECX + 0x2c) = 0;
    Ui_Widget::retranslateUi(in_ECX);
    local_28[0] = (*local_48)();
    pQVar3 = (QString *)0x5dce3;
    (*___imp___ZN15QAbstractButton7setTextERK7QString)();
  }
  else {
    if (iVar1 == 0) {
      local_5c = (undefined4 *)0xf;
      local_60 = (undefined4 *)&DAT_00002d8d;
      local_28[0] = ___imp___ZN10QArrayData11shared_nullE;
      local_2c = ___imp___ZN10QArrayData11shared_nullE;
      local_30 = ___imp___ZN10QArrayData11shared_nullE;
      local_48 = ___imp___ZN7QString16fromAscii_helperEPKci;
      local_34 = (*___imp___ZN7QString16fromAscii_helperEPKci)();
      local_60 = &local_34;
      local_54 = local_28;
      local_58 = &local_2c;
      local_5c = &local_30;
      local_44 = local_60;
      local_40 = local_58;
      (*___imp___ZN11QTranslator4loadERK7QStringS2_S2_S2_)();
      QString::~QString(in_stack_ffffff90);
      QString::~QString(in_stack_ffffff90);
      QString::~QString(in_stack_ffffff90);
      QString::~QString(in_stack_ffffff90);
      (*___imp___ZN16QCoreApplication17installTranslatorEP11QTranslator)();
      *(undefined4 *)(in_ECX + 0x2c) = 1;
      Ui_Widget::retranslateUi(in_ECX);
      local_28[0] = (*local_48)();
      pQVar3 = (QString *)0x5dbfe;
      (*___imp___ZN15QAbstractButton7setTextERK7QString)();
LAB_0005dbfe:
      ppuVar2 = (undefined4 **)&stack0xffffff88;
      QString::~QString(pQVar3);
      InitBasicEn();
      goto LAB_0005db2b;
    }
    ppuVar2 = &local_60;
    if (iVar1 != 1) goto LAB_0005db2b;
    local_5c = (undefined4 *)0xf;
    local_60 = (undefined4 *)&DAT_00002d9d;
    local_28[0] = ___imp___ZN10QArrayData11shared_nullE;
    local_2c = ___imp___ZN10QArrayData11shared_nullE;
    local_30 = ___imp___ZN10QArrayData11shared_nullE;
    local_48 = ___imp___ZN7QString16fromAscii_helperEPKci;
    local_34 = (*___imp___ZN7QString16fromAscii_helperEPKci)();
    local_60 = &local_34;
    local_54 = local_28;
    local_58 = &local_2c;
    local_5c = &local_30;
    local_44 = local_60;
    local_40 = local_58;
    (*___imp___ZN11QTranslator4loadERK7QStringS2_S2_S2_)();
    QString::~QString(in_stack_ffffff90);
    QString::~QString(in_stack_ffffff90);
    QString::~QString(in_stack_ffffff90);
    QString::~QString(in_stack_ffffff90);
    (*___imp___ZN16QCoreApplication17installTranslatorEP11QTranslator)();
    *(undefined4 *)(in_ECX + 0x2c) = 0;
    Ui_Widget::retranslateUi(in_ECX);
    local_28[0] = (*local_48)();
    pQVar3 = (QString *)0x5de81;
    (*___imp___ZN15QAbstractButton7setTextERK7QString)();
  }
  ppuVar2 = (undefined4 **)&stack0xffffff88;
  QString::~QString(pQVar3);
  InitBasic();
LAB_0005db2b:
  *(undefined4 *)((int)ppuVar2 + -4) = 0x5db34;
  (*___imp___ZN11QTranslatorD1Ev)();
  return;
}

