
void FUN_00403eb0(void)

{
  QObject *pQVar1;
  int iVar2;
  char cVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  QArrayData *local_40;
  QTextStream *local_3c;
  QArrayData *local_38;
  QArrayData *local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  char *local_20;
  
  if (DAT_0049e0fe == '\b') {
    return;
  }
  pQVar1 = (QObject *)QButtonGroup::checkedButton();
  QMetaObject::cast(pQVar1);
  QObject::objectName();
  local_30 = 2;
  local_2c = 0;
  local_28 = 0;
  local_24 = 0;
  local_20 = "default";
  QMessageLogger::debug();
  uVar5 = 0xffffffff;
  uVar4 = 0;
  QMetaObject::tr(&DAT_0049e138,&DAT_0049f534,0);
  QString::arg(&local_38,&local_40,0,CONCAT22(uVar4,0x20),uVar5);
  QDebug::putString((QChar *)(local_34 + *(int *)(local_34 + 0xc)),*(uint *)(local_34 + 4));
  if (local_3c[0x14] != (QTextStream)0x0) {
    QTextStream::operator<<(local_3c,' ');
  }
  if (*(int *)local_34 == 0) {
LAB_00404080:
    QArrayData::deallocate(local_34,2,4);
    iVar2 = *(int *)local_38;
    if (iVar2 != 0) goto LAB_00403fd3;
LAB_004040a6:
    QArrayData::deallocate(local_38,2,4);
  }
  else {
    if (*(int *)local_34 != -1) {
      LOCK();
      *(int *)local_34 = *(int *)local_34 + -1;
      UNLOCK();
      if (*(int *)local_34 == 0) goto LAB_00404080;
    }
    iVar2 = *(int *)local_38;
    if (iVar2 == 0) goto LAB_004040a6;
LAB_00403fd3:
    if (iVar2 != -1) {
      LOCK();
      *(int *)local_38 = *(int *)local_38 + -1;
      UNLOCK();
      if (*(int *)local_38 == 0) goto LAB_004040a6;
    }
  }
  QDebug::~QDebug((QDebug *)&local_3c);
  iVar2 = QString::compare_helper
                    (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4),
                     "pushButton_K1",0xffffffff,1);
  if (iVar2 == 0) {
    DAT_00620474 = 1;
    iVar2 = (uint)DAT_00620475 * 3000;
    cVar3 = DAT_00620475 + 1;
    (&DAT_0061e172)[iVar2] = 0xfd;
    (&DAT_0061e173)[iVar2] = 1;
    (&DAT_0061e174)[iVar2] = cVar3;
    FUN_0040c2d0();
  }
  else {
    iVar2 = QString::compare_helper
                      (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4),
                       "pushButton_K2",0xffffffff,1);
    if (iVar2 == 0) {
      DAT_00620474 = 2;
      iVar2 = (uint)DAT_00620475 * 3000;
      cVar3 = DAT_00620475 + 1;
      (&DAT_0061e1a4)[iVar2] = 0xfd;
      (&DAT_0061e1a5)[iVar2] = 2;
      (&DAT_0061e1a6)[iVar2] = cVar3;
      FUN_0040c2d0();
    }
    else {
      iVar2 = QString::compare_helper
                        (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4),
                         "pushButton_K3",0xffffffff,1);
      if (iVar2 == 0) {
        DAT_00620474 = 3;
        iVar2 = (uint)DAT_00620475 * 3000;
        cVar3 = DAT_00620475 + 1;
        (&DAT_0061e1d6)[iVar2] = 0xfd;
        (&DAT_0061e1d7)[iVar2] = 3;
        (&DAT_0061e1d8)[iVar2] = cVar3;
        FUN_0040c2d0();
      }
      else {
        iVar2 = QString::compare_helper
                          (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4),
                           "pushButton_K4",0xffffffff,1);
        if (iVar2 == 0) {
          DAT_00620474 = 4;
          iVar2 = (uint)DAT_00620475 * 3000;
          cVar3 = DAT_00620475 + 1;
          (&DAT_0061e208)[iVar2] = 0xfd;
          (&DAT_0061e209)[iVar2] = 4;
          (&DAT_0061e20a)[iVar2] = cVar3;
          FUN_0040c2d0();
        }
        else {
          iVar2 = QString::compare_helper
                            (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4),
                             "pushButton_K5",0xffffffff,1);
          if (iVar2 == 0) {
            DAT_00620474 = 5;
            iVar2 = (uint)DAT_00620475 * 3000;
            cVar3 = DAT_00620475 + 1;
            (&DAT_0061e23a)[iVar2] = 0xfd;
            (&DAT_0061e23b)[iVar2] = 5;
            (&DAT_0061e23c)[iVar2] = cVar3;
            FUN_0040c2d0();
          }
          else {
            iVar2 = QString::compare_helper
                              (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4),
                               "pushButton_K6",0xffffffff,1);
            if (iVar2 == 0) {
              DAT_00620474 = 6;
              iVar2 = (uint)DAT_00620475 * 3000;
              cVar3 = DAT_00620475 + 1;
              (&DAT_0061e26c)[iVar2] = 0xfd;
              (&DAT_0061e26d)[iVar2] = 6;
              (&DAT_0061e26e)[iVar2] = cVar3;
              FUN_0040c2d0();
            }
            else {
              iVar2 = QString::compare_helper
                                (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4),
                                 "pushButton_K7",0xffffffff,1);
              if (iVar2 == 0) {
                DAT_00620474 = 7;
                iVar2 = (uint)DAT_00620475 * 3000;
                cVar3 = DAT_00620475 + 1;
                (&DAT_0061e29e)[iVar2] = 0xfd;
                (&DAT_0061e29f)[iVar2] = 7;
                (&DAT_0061e2a0)[iVar2] = cVar3;
                FUN_0040c2d0();
              }
              else {
                iVar2 = QString::compare_helper
                                  (local_40 + *(int *)(local_40 + 0xc),*(undefined4 *)(local_40 + 4)
                                   ,"pushButton_K8",0xffffffff,1);
                if (iVar2 == 0) {
                  DAT_00620474 = 8;
                  iVar2 = (uint)DAT_00620475 * 3000;
                  cVar3 = DAT_00620475 + 1;
                  (&DAT_0061e2d0)[iVar2] = 0xfd;
                  (&DAT_0061e2d1)[iVar2] = 8;
                  (&DAT_0061e2d2)[iVar2] = cVar3;
                  FUN_0040c2d0();
                }
                else {
                  cVar3 = FUN_00493910("pushButton_K9",&local_40);
                  if (cVar3 == '\0') {
                    cVar3 = FUN_00493910("pushButton_K10",&local_40);
                    if (cVar3 == '\0') {
                      cVar3 = FUN_00493910("pushButton_K11",&local_40);
                      if (cVar3 == '\0') {
                        cVar3 = FUN_00493910("pushButton_K12",&local_40);
                        if (cVar3 == '\0') {
                          cVar3 = FUN_00493910("pushButton_K13",&local_40);
                          if (cVar3 == '\0') {
                            cVar3 = FUN_00493910("pushButton_K14",&local_40);
                            if (cVar3 == '\0') {
                              cVar3 = FUN_00493910("pushButton_K15",&local_40);
                              if (cVar3 == '\0') {
                                cVar3 = FUN_00493910("k1_left",&local_40);
                                if (cVar3 == '\0') {
                                  cVar3 = FUN_00493910("k1_middle",&local_40);
                                  if (cVar3 == '\0') {
                                    cVar3 = FUN_00493910("k1_right",&local_40);
                                    if (cVar3 == '\0') {
                                      cVar3 = FUN_00493910("k2_left",&local_40);
                                      if (cVar3 == '\0') {
                                        cVar3 = FUN_00493910("k2_middle",&local_40);
                                        if (cVar3 == '\0') {
                                          cVar3 = FUN_00493910("k2_right",&local_40);
                                          if (cVar3 == '\0') {
                                            cVar3 = FUN_00493910("k3_left",&local_40);
                                            if (cVar3 == '\0') {
                                              cVar3 = FUN_00493910("k3_middle",&local_40);
                                              if (cVar3 == '\0') {
                                                cVar3 = FUN_00493910("k3_right",&local_40);
                                                if (cVar3 == '\0') {
                                                  cVar3 = FUN_00493910("k4_left",&local_40);
                                                  if (cVar3 == '\0') {
                                                    cVar3 = FUN_00493910("k4_middle",&local_40);
                                                    if (cVar3 == '\0') {
                                                      cVar3 = FUN_00493910("k4_right",&local_40);
                                                      if (cVar3 != '\0') {
                                                        DAT_00620474 = 0x1b;
                                                        iVar2 = (uint)DAT_00620475 * 3000;
                                                        cVar3 = DAT_00620475 + 1;
                                                        (&DAT_0061e686)[iVar2] = 0xfd;
                                                        (&DAT_0061e687)[iVar2] = 0x1b;
                                                        (&DAT_0061e688)[iVar2] = cVar3;
                                                        FUN_0040c2d0();
                                                      }
                                                    }
                                                    else {
                                                      DAT_00620474 = 0x1a;
                                                      iVar2 = (uint)DAT_00620475 * 3000;
                                                      cVar3 = DAT_00620475 + 1;
                                                      (&DAT_0061e654)[iVar2] = 0xfd;
                                                      (&DAT_0061e655)[iVar2] = 0x1a;
                                                      (&DAT_0061e656)[iVar2] = cVar3;
                                                      FUN_0040c2d0();
                                                    }
                                                  }
                                                  else {
                                                    DAT_00620474 = 0x19;
                                                    iVar2 = (uint)DAT_00620475 * 3000;
                                                    cVar3 = DAT_00620475 + 1;
                                                    (&DAT_0061e622)[iVar2] = 0xfd;
                                                    (&DAT_0061e623)[iVar2] = 0x19;
                                                    (&DAT_0061e624)[iVar2] = cVar3;
                                                    FUN_0040c2d0();
                                                  }
                                                }
                                                else {
                                                  DAT_00620474 = 0x18;
                                                  iVar2 = (uint)DAT_00620475 * 3000;
                                                  cVar3 = DAT_00620475 + 1;
                                                  (&DAT_0061e5f0)[iVar2] = 0xfd;
                                                  (&DAT_0061e5f1)[iVar2] = 0x18;
                                                  (&DAT_0061e5f2)[iVar2] = cVar3;
                                                  FUN_0040c2d0();
                                                }
                                              }
                                              else {
                                                DAT_00620474 = 0x17;
                                                iVar2 = (uint)DAT_00620475 * 3000;
                                                cVar3 = DAT_00620475 + 1;
                                                (&DAT_0061e5be)[iVar2] = 0xfd;
                                                (&DAT_0061e5bf)[iVar2] = 0x17;
                                                (&DAT_0061e5c0)[iVar2] = cVar3;
                                                FUN_0040c2d0();
                                              }
                                            }
                                            else {
                                              DAT_00620474 = 0x16;
                                              iVar2 = (uint)DAT_00620475 * 3000;
                                              cVar3 = DAT_00620475 + 1;
                                              (&DAT_0061e58c)[iVar2] = 0xfd;
                                              (&DAT_0061e58d)[iVar2] = 0x16;
                                              (&DAT_0061e58e)[iVar2] = cVar3;
                                              FUN_0040c2d0();
                                            }
                                          }
                                          else {
                                            DAT_00620474 = 0x15;
                                            iVar2 = (uint)DAT_00620475 * 3000;
                                            cVar3 = DAT_00620475 + 1;
                                            (&DAT_0061e55a)[iVar2] = 0xfd;
                                            (&DAT_0061e55b)[iVar2] = 0x15;
                                            (&DAT_0061e55c)[iVar2] = cVar3;
                                            FUN_0040c2d0();
                                          }
                                        }
                                        else {
                                          DAT_00620474 = 0x14;
                                          iVar2 = (uint)DAT_00620475 * 3000;
                                          cVar3 = DAT_00620475 + 1;
                                          (&DAT_0061e528)[iVar2] = 0xfd;
                                          (&DAT_0061e529)[iVar2] = 0x14;
                                          (&DAT_0061e52a)[iVar2] = cVar3;
                                          FUN_0040c2d0();
                                        }
                                      }
                                      else {
                                        DAT_00620474 = 0x13;
                                        iVar2 = (uint)DAT_00620475 * 3000;
                                        cVar3 = DAT_00620475 + 1;
                                        (&DAT_0061e4f6)[iVar2] = 0xfd;
                                        (&DAT_0061e4f7)[iVar2] = 0x13;
                                        (&DAT_0061e4f8)[iVar2] = cVar3;
                                        FUN_0040c2d0();
                                      }
                                    }
                                    else {
                                      DAT_00620474 = 0x12;
                                      iVar2 = (uint)DAT_00620475 * 3000;
                                      cVar3 = DAT_00620475 + 1;
                                      (&DAT_0061e4c4)[iVar2] = 0xfd;
                                      (&DAT_0061e4c5)[iVar2] = 0x12;
                                      (&DAT_0061e4c6)[iVar2] = cVar3;
                                      FUN_0040c2d0();
                                    }
                                  }
                                  else {
                                    DAT_00620474 = 0x11;
                                    iVar2 = (uint)DAT_00620475 * 3000;
                                    cVar3 = DAT_00620475 + 1;
                                    (&DAT_0061e492)[iVar2] = 0xfd;
                                    (&DAT_0061e493)[iVar2] = 0x11;
                                    (&DAT_0061e494)[iVar2] = cVar3;
                                    FUN_0040c2d0();
                                  }
                                }
                                else {
                                  DAT_00620474 = 0x10;
                                  iVar2 = (uint)DAT_00620475 * 3000;
                                  cVar3 = DAT_00620475 + 1;
                                  (&DAT_0061e460)[iVar2] = 0xfd;
                                  (&DAT_0061e461)[iVar2] = 0x10;
                                  (&DAT_0061e462)[iVar2] = cVar3;
                                  FUN_0040c2d0();
                                }
                              }
                              else {
                                DAT_00620474 = 0xf;
                                iVar2 = (uint)DAT_00620475 * 3000;
                                cVar3 = DAT_00620475 + 1;
                                (&DAT_0061e42e)[iVar2] = 0xfd;
                                (&DAT_0061e42f)[iVar2] = 0xf;
                                (&DAT_0061e430)[iVar2] = cVar3;
                                FUN_0040c2d0();
                              }
                            }
                            else {
                              DAT_00620474 = 0xe;
                              iVar2 = (uint)DAT_00620475 * 3000;
                              cVar3 = DAT_00620475 + 1;
                              (&DAT_0061e3fc)[iVar2] = 0xfd;
                              (&DAT_0061e3fd)[iVar2] = 0xe;
                              (&DAT_0061e3fe)[iVar2] = cVar3;
                              FUN_0040c2d0();
                            }
                          }
                          else {
                            DAT_00620474 = 0xd;
                            iVar2 = (uint)DAT_00620475 * 3000;
                            cVar3 = DAT_00620475 + 1;
                            (&DAT_0061e3ca)[iVar2] = 0xfd;
                            (&DAT_0061e3cb)[iVar2] = 0xd;
                            (&DAT_0061e3cc)[iVar2] = cVar3;
                            FUN_0040c2d0();
                          }
                        }
                        else {
                          DAT_00620474 = 0xc;
                          iVar2 = (uint)DAT_00620475 * 3000;
                          cVar3 = DAT_00620475 + 1;
                          (&DAT_0061e398)[iVar2] = 0xfd;
                          (&DAT_0061e399)[iVar2] = 0xc;
                          (&DAT_0061e39a)[iVar2] = cVar3;
                          FUN_0040c2d0();
                        }
                      }
                      else {
                        DAT_00620474 = 0xb;
                        iVar2 = (uint)DAT_00620475 * 3000;
                        cVar3 = DAT_00620475 + 1;
                        (&DAT_0061e366)[iVar2] = 0xfd;
                        (&DAT_0061e367)[iVar2] = 0xb;
                        (&DAT_0061e368)[iVar2] = cVar3;
                        FUN_0040c2d0();
                      }
                    }
                    else {
                      DAT_00620474 = 10;
                      iVar2 = (uint)DAT_00620475 * 3000;
                      cVar3 = DAT_00620475 + 1;
                      (&DAT_0061e334)[iVar2] = 0xfd;
                      (&DAT_0061e335)[iVar2] = 10;
                      (&DAT_0061e336)[iVar2] = cVar3;
                      FUN_0040c2d0();
                    }
                  }
                  else {
                    DAT_00620474 = 9;
                    iVar2 = (uint)DAT_00620475 * 3000;
                    cVar3 = DAT_00620475 + 1;
                    (&DAT_0061e302)[iVar2] = 0xfd;
                    (&DAT_0061e303)[iVar2] = 9;
                    (&DAT_0061e304)[iVar2] = cVar3;
                    FUN_0040c2d0();
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  if (*(int *)local_40 != 0) {
    if (*(int *)local_40 != -1) {
      LOCK();
      *(int *)local_40 = *(int *)local_40 + -1;
      UNLOCK();
      if (*(int *)local_40 == 0) goto LAB_004040c4;
    }
    return;
  }
LAB_004040c4:
  QArrayData::deallocate(local_40,2,4);
  return;
}

