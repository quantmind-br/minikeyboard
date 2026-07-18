
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* Widget::Identify_KeyBoard_style() */

void Widget::Identify_KeyBoard_style(void)

{
  code *UNRECOVERED_JUMPTABLE;
  
  if (DAT_000000db == '\0') {
    if (DAT_000000dc != '\x01') {
      if (DAT_000000dc != '\x02') {
        if (DAT_000000dc == '\x03') {
          Set_Keyboard_0add3();
          goto LAB_0005c806;
        }
        goto LAB_0005c7ff;
      }
      if (_DAT_000000f8 == -0x77b0) {
        Set_Keyboard_0add2();
        goto LAB_0005c806;
      }
    }
    Set_Keyboard_0add1();
    goto LAB_0005c806;
  }
  if (DAT_000000db == '\x01') {
    if (DAT_000000dc == '\0') {
      Set_Keyboard_1add0();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\x02') {
    if (DAT_000000dc == '\0') {
      Set_Keyboard_2add0();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\x03') {
    if (DAT_000000dc == '\0') {
      Set_Keyboard_3add0();
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x01') {
      Set_Keyboard_3add1();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\x04') {
    if (DAT_000000dc == '\0') {
      Set_Keyboard_4Key();
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x01') {
      if (DAT_000000dd == '\0') {
        if (_DAT_000000f8 != -0x77af) {
          Set_Keyboard_4add1();
          goto LAB_0005c806;
        }
      }
      else if (DAT_000000dd != '\x01') goto LAB_0005c7ff;
      Set_Keyboard_4add2();
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x03') {
      Set_Keyboard_4add3();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\x05') {
    if (DAT_000000dc == '\0') {
      Set_Keyboard_5Key_Mute();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\x06') {
    if (DAT_000000dc == '\0') {
      Set_Keyboard_6Key();
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x01') {
      Set_Keyboard_6add1();
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x02') {
      if (DAT_00002436 == '\x01') {
        Set_Keyboard_6add2_Lan_KD();
      }
      else {
        Set_Keyboard_6add2();
      }
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\v') {
    if (DAT_000000dc == '\x03') {
      Set_Keyboard_11add3();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\f') {
    if (DAT_000000dc == '\x03') {
      Set_Keyboard_12add3();
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x04') {
      if (DAT_00002436 == '\x01') {
        Set_Keyboard_12add4_Lan();
      }
      else {
        Set_Keyboard_12add4();
      }
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x02') {
      Set_Keyboard_12add2();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\t') {
    if (DAT_000000dc == '\x02') {
      Set_Keyboard_9add2();
      goto LAB_0005c806;
    }
    if (DAT_000000dc == '\x03') {
      Set_Keyboard_9add3();
      goto LAB_0005c806;
    }
  }
  else if (DAT_000000db == '\x10') {
    if (DAT_000000dc == '\0') {
      Set_Keyboard_16add0();
      goto LAB_0005c806;
    }
  }
  else if ((DAT_000000db == '\x15') && (DAT_000000dc == '\x01')) {
    Set_Keyboard_21add1();
    goto LAB_0005c806;
  }
LAB_0005c7ff:
  Set_Keyboard_15add3();
LAB_0005c806:
  UNRECOVERED_JUMPTABLE = ___imp___ZN7QWidget4hideEv;
  (*___imp___ZN7QWidget4hideEv)();
  (*UNRECOVERED_JUMPTABLE)();
  (*UNRECOVERED_JUMPTABLE)();
  (*UNRECOVERED_JUMPTABLE)();
  (*UNRECOVERED_JUMPTABLE)();
                    /* WARNING: Could not recover jumptable at 0x0005c853. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*UNRECOVERED_JUMPTABLE)();
  return;
}

