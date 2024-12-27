from manim import *

class InPlace(Scene):
    def construct(self):
        messages=[
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
        font="Liberation Mono"
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
