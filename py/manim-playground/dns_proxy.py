from manim import *

FONT = "Liberation Mono"

YELLOW = "#ffb000"
PINK = "#dc267f"
PURPLE = "#785ef0"

class Device:
    def __init__(self, name):
        self.name = name

    def to_mobj(self):
        text = Text(self.name, font=FONT, font_size=24)
        border = SurroundingRectangle(text, WHITE)
        return VGroup(text, border)

class DNSQuery:
    def __init__(self, payload, dAddr, box_color=YELLOW, text_color=WHITE):
        self.payload = payload
        self.dAddr = dAddr
        self.box_color = box_color
        self.text_color = text_color

    def to_mobj(self):
        text = "{}\ndst: {}".format(self.payload, self.dAddr)
        text = Text(text, font=FONT, font_size=24, color=self.text_color).align_to(LEFT)
        border = SurroundingRectangle(text, self.box_color)
        return VGroup(text, border)

    def timed_out(self):
        return DNSQuery(self.payload, self.dAddr, box_color=GRAY, text_color=GRAY)

class DNSResponse:
    def __init__(self, payload):
        self.payload = payload

    def to_mobj(self):
        text = self.payload
        text = Text(text, font=FONT, font_size=24).align_to(LEFT)
        border = SurroundingRectangle(text, PURPLE)
        return VGroup(text, border)

class DnsDistProxyDemo(Scene):
    def caption(self, text):
        return Text(text, font=FONT, font_size=18).to_corner(DL, buff=0.25)

    def construct(self):
        # Display client and DNS proxy
        original_q = DNSQuery("A? example.org.", "192.0.2.53")
        packet_in = original_q.to_mobj().shift(5.5 * LEFT)
        sender = Device("Client").to_mobj().next_to(packet_in, UP)
        packet_at_proxy = original_q.to_mobj().next_to(packet_in, RIGHT)
        proxy = Device("DNS proxy\n192.0.2.53:53").to_mobj().next_to(packet_at_proxy, UP)
        self.add(sender)
        self.add(proxy)
        self.wait(1)
        # Client makes a request
        caption_1 = self.caption("A DNS client looks up the domain example.org").shift(1.00 * UP)
        self.play(FadeIn(packet_in), FadeIn(caption_1))
        self.wait(5)

        # Request moves to proxy
        caption_2 = self.caption("The client is configured with the DNS proxy (192.0.2.53) as its resolver,\nso the query is sent to 192.0.2.53.")
        self.play(Transform(packet_in, packet_at_proxy), FadeIn(caption_2))
        self.wait(10)

        # Proxy is configured with two upstream resolvers
        caption_3 = self.caption("DNS proxy is configured with two downstream resolvers:\n1.1.1.1 and 8.8.8.8").shift(0.8 * UP)
        public_resolver_1 = Device("Resolver 1\n1.1.1.1").to_mobj().shift(4 * RIGHT).shift(3 * UP)
        public_resolver_2 = Device("Resolver 2\n8.8.8.8").to_mobj().shift(4 * RIGHT).shift(2.75 * DOWN)
        public_resolvers = VGroup(public_resolver_1, public_resolver_2)
        self.play(
            FadeIn(public_resolvers),
            FadeIn(caption_3),
            FadeOut(caption_1),
            FadeOut(caption_2),
        )
        self.wait(8)

        # Proxy sends a request to both resolvers
        caption_4 = self.caption("...and the DNS proxy is configured to send queries concurrently to multiple resolvers.")
        packet_to_resolver_1 = DNSQuery("A? example.org.", "1.1.1.1").to_mobj().next_to(packet_at_proxy, DOWN)
        packet_to_resolver_2 = DNSQuery("A? example.org.", "8.8.8.8").to_mobj().next_to(packet_to_resolver_1, DOWN) 
        self.play(
            FadeIn(packet_to_resolver_1),
            FadeIn(packet_to_resolver_2),
            FadeOut(packet_in),
            FadeIn(caption_4),
        )
        self.wait(8)

        # Requests are sent concurrently
        packet_at_resolver_1 = packet_to_resolver_1.copy().next_to(public_resolver_1, DOWN)
        packet_at_resolver_2 = packet_to_resolver_2.copy().next_to(public_resolver_2, UP)
        self.play(
            Transform(packet_to_resolver_1, packet_at_resolver_1),
            Transform(packet_to_resolver_2, packet_at_resolver_2),
        )
        self.wait(3)

        # Resolver 1 fails to answer
        caption_5 = self.caption("Resolver 1 fails to respond").next_to(public_resolver_1, LEFT)
        resolver_1_fail = DNSQuery("A? example.org.", "1.1.1.1").timed_out().to_mobj().next_to(public_resolver_1, DOWN)
        self.play(
            Transform(packet_at_resolver_1, resolver_1_fail),
            FadeIn(caption_5),
            FadeOut(caption_3),
            FadeOut(caption_4),
        )
        self.wait(2)

        # Resolver 2 answers
        caption_6 = self.caption("...but Resolver 2 does respond").next_to(public_resolver_2, LEFT)
        dns_response_text = "example.org:\nA 93.184.215.14"
        self.remove(packet_to_resolver_2)
        response_resolver_2 = DNSResponse(dns_response_text).to_mobj().next_to(public_resolver_2, UP)
        self.play(
            Transform(packet_at_resolver_2, response_resolver_2),
            FadeIn(caption_6),
        )
        self.wait(7)

        # Resolver 2 answer arrives at proxy
        self.remove(packet_at_resolver_2)
        response_at_proxy = DNSResponse(dns_response_text).to_mobj().next_to(proxy, DOWN)
        self.play(
            Transform(response_resolver_2, response_at_proxy),
            FadeOut(caption_5),
        )
        self.wait(3)

        # Proxy forwards answer back to client
        caption_7 = self.caption("Finally, the Proxy forwards the response back to the original client")
        self.remove(response_resolver_2)
        response_to_client = DNSResponse(dns_response_text).to_mobj().next_to(sender, DOWN)
        self.play(
            Transform(response_at_proxy, response_to_client),
            FadeIn(caption_7),
            FadeOut(caption_6),
        )
        self.wait(10)
