from manim import *
        
quick_brown_fox=[
    'THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG',
    'UIF RVJDL CSPXO GPY KVNQT PWFS UIF MBAZ EPH',
    'VJG SWKEM DTQYP HQZ LWORU QXGT VJG NCBA FQI',
    'WKH TXLFN EURZQ IRA MXPSV RYHU WKH ODCB GRJ',
    'XLI UYMGO FVSAR JSB NYQTW SZIV XLI PEDC HSK',
    'YMJ VZNHP GWTBS KTC OZRUX TAJW YMJ QFED ITL',
    'ZNK WAOIQ HXUCT LUD PASVY UBKX ZNK RGFE JUM',
    'AOL XBPJR IYVDU MVE QBTWZ VCLY AOL SHGF KVN',
    'BPM YCQKS JZWEV NWF RCUXA WDMZ BPM TIHG LWO',
    'CQN ZDRLT KAXFW OXG SDVYB XENA CQN UJIH MXP',
    'DRO AESMU LBYGX PYH TEWZC YFOB DRO VKJI NYQ',
    'ESP BFTNV MCZHY QZI UFXAD ZGPC ESP WLKJ OZR',
    'FTQ CGUOW NDAIZ RAJ VGYBE AHQD FTQ XMLK PAS',
    'GUR DHVPX OEBJA SBK WHZCF BIRE GUR YNML QBT',
    'HVS EIWQY PFCKB TCL XIADG CJSF HVS ZONM RCU',
    'IWT FJXRZ QGDLC UDM YJBEH DKTG IWT APON SDV',
    'JXU GKYSA RHEMD VEN ZKCFI ELUH JXU BQPO TEW',
    'KYV HLZTB SIFNE WFO ALDGJ FMVI KYV CRQP UFX',
    'LZW IMAUC TJGOF XGP BMEHK GNWJ LZW DSRQ VGY',
    'MAX JNBVD UKHPG YHQ CNFIL HOXK MAX ETSR WHZ',
    'NBY KOCWE VLIQH ZIR DOGJM IPYL NBY FUTS XIA',
    'OCZ LPDXF WMJRI AJS EPHKN JQZM OCZ GVUT YJB',
    'PDA MQEYG XNKSJ BKT FQILO KRAN PDA HWVU ZKC',
    'QEB NRFZH YOLTK CLU GRJMP LSBO QEB IXWV ALD',
    'RFC OSGAI ZPMUL DMV HSKNQ MTCP RFC JYXW BME',
    'SGD PTHBJ AQNVM ENW ITLOR NUDQ SGD KZYX CNF',
    'THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG',
]

sphinx=[
    'SPHINX OF BLACK QUARTZ, JUDGE MY VOW.',
    'TQIJOY PG CMBDL RVBSUA, KVEHF NZ WPX.',
    'URJKPZ QH DNCEM SWCTVB, LWFIG OA XQY.',
    'VSKLQA RI EODFN TXDUWC, MXGJH PB YRZ.',
    'WTLMRB SJ FPEGO UYEVXD, NYHKI QC ZSA.',
    'XUMNSC TK GQFHP VZFWYE, OZILJ RD ATB.',
    'YVNOTD UL HRGIQ WAGXZF, PAJMK SE BUC.',
    'ZWOPUE VM ISHJR XBHYAG, QBKNL TF CVD.',
    'AXPQVF WN JTIKS YCIZBH, RCLOM UG DWE.',
    'BYQRWG XO KUJLT ZDJACI, SDMPN VH EXF.',
    'CZRSXH YP LVKMU AEKBDJ, TENQO WI FYG.',
    'DASTYI ZQ MWLNV BFLCEK, UFORP XJ GZH.',
    'EBTUZJ AR NXMOW CGMDFL, VGPSQ YK HAI.',
    'FCUVAK BS OYNPX DHNEGM, WHQTR ZL IBJ.',
    'GDVWBL CT PZOQY EIOFHN, XIRUS AM JCK.',
    'HEWXCM DU QAPRZ FJPGIO, YJSVT BN KDL.',
    'IFXYDN EV RBQSA GKQHJP, ZKTWU CO LEM.',
    'JGYZEO FW SCRTB HLRIKQ, ALUXV DP MFN.',
    'KHZAFP GX TDSUC IMSJLR, BMVYW EQ NGO.',
    'LIABGQ HY UETVD JNTKMS, CNWZX FR OHP.',
    'MJBCHR IZ VFUWE KOULNT, DOXAY GS PIQ.',
    'NKCDIS JA WGVXF LPVMOU, EPYBZ HT QJR.',
    'OLDEJT KB XHWYG MQWNPV, FQZCA IU RKS.',
    'PMEFKU LC YIXZH NRXOQW, GRADB JV SLT.',
    'QNFGLV MD ZJYAI OSYPRX, HSBEC KW TMU.',
    'ROGHMW NE AKZBJ PTZQSY, ITCFD LX UNV.',
    'SPHINX OF BLACK QUARTZ, JUDGE MY VOW.',
]

class InPlace(Scene):
    def construct(self):
        font="Liberation Mono"
        messages = quick_brown_fox
        render_text = lambda msg: Text(msg, font_size="38", font=font)
        render_subtitle = lambda msg: Text(msg, font_size="34", font=font, color="#888888").shift(0.5 * DOWN)
        texts = list(map(render_text, messages))
        current_text = texts[0]
        current_subtitle = render_subtitle("Original Text")
        self.add(current_text)
        self.add(current_subtitle)
        self.wait()
        rot_count = 0
        for next_text in texts[1:]:
          rot_count = rot_count + 1
          next_subtitle = render_subtitle("ROT%d" % rot_count)
          self.play(Transform(current_subtitle, next_subtitle))
          self.play(Transform(current_text, next_text))

        final_subtitle = render_subtitle("Original Text")
        self.play(Transform(current_subtitle, final_subtitle))

class StackText(Scene):
    def construct(self):
        font="Liberation Mono"
        messages = sphinx
        render_text = lambda msg: Text(msg, font_size="18", font=font)
        texts = list(map(render_text, messages))
        initial_position = 3.65 * UP
        current_text = texts[0].shift(initial_position)
        self.add(current_text)
        self.wait()
        for next_text in texts[1:]:
          next_text.next_to(current_text, DOWN, buff=0.06)
          self.play(AddTextLetterByLetter(next_text, run_time=0.5))
          current_text = next_text
          self.wait()
        self.wait(5)
