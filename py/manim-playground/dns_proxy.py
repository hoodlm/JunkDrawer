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
    def construct(self):
        # Display client and DNS proxy
        original_q = DNSQuery("A? example.org.", "192.0.2.53")
        packet_in = original_q.to_mobj().shift(5.5 * LEFT)
        sender = Device("Client").to_mobj().next_to(packet_in, UP)
        packet_at_proxy = original_q.to_mobj().next_to(packet_in, RIGHT)
        proxy = Device("dnsdist Proxy\n192.0.2.53:53").to_mobj().next_to(packet_at_proxy, UP)
        self.add(sender)
        self.add(proxy)
        # Client makes a request
        self.wait(1)
        self.play(FadeIn(packet_in))
        self.wait(3)

        # Request moves to proxy
        self.wait(3)
        self.play(Transform(packet_in, packet_at_proxy))

        # Proxy is configured with two upstream resolvers
        self.wait(1)
        public_resolver_1 = Device("Resolver 1\n1.1.1.1").to_mobj().shift(4 * RIGHT).shift(3 * UP)
        public_resolver_2 = Device("Resolver 2\n8.8.8.8").to_mobj().shift(4 * RIGHT).shift(3 * DOWN)
        public_resolvers = VGroup(public_resolver_1, public_resolver_2)
        self.play(FadeIn(public_resolvers))
        self.wait(3)

        # Proxy sends a request to both resolvers
        packet_to_resolver_1 = DNSQuery("A? example.org.", "1.1.1.1").to_mobj().next_to(packet_at_proxy, DOWN)
        packet_to_resolver_2 = DNSQuery("A? example.org.", "8.8.8.8").to_mobj().next_to(packet_to_resolver_1, DOWN) 
        self.play(
            FadeIn(packet_to_resolver_1),
            FadeIn(packet_to_resolver_2),
            FadeOut(packet_in),
        )
        self.wait(3)

        # Requests are sent concurrently
        packet_at_resolver_1 = packet_to_resolver_1.copy().next_to(public_resolver_1, DOWN)
        packet_at_resolver_2 = packet_to_resolver_2.copy().next_to(public_resolver_2, UP)
        self.play(
            Transform(packet_to_resolver_1, packet_at_resolver_1),
            Transform(packet_to_resolver_2, packet_at_resolver_2),
        )
        self.wait(3)

        # Resolver 1 fails to answer
        resolver_1_fail = DNSQuery("A? example.org.", "1.1.1.1").timed_out().to_mobj().next_to(public_resolver_1, DOWN)
        self.play(Transform(packet_at_resolver_1, resolver_1_fail))
        self.wait(1)

        # Resolver 2 answers
        dns_response_text = "example.org:\nA 93.184.215.14"
        self.remove(packet_to_resolver_2)
        response_resolver_2 = DNSResponse(dns_response_text).to_mobj().next_to(public_resolver_2, UP)
        self.play(
            Transform(packet_at_resolver_2, response_resolver_2),
        )
        self.wait(3)

        # Resolver 2 answer arrives at proxy
        self.remove(packet_at_resolver_2)
        response_at_proxy = DNSResponse(dns_response_text).to_mobj().next_to(proxy, DOWN)
        self.play(
            Transform(response_resolver_2, response_at_proxy),
        )
        self.wait(3)

        # Proxy forwards answer back to client
        self.remove(response_resolver_2)
        response_to_client = DNSResponse(dns_response_text).to_mobj().next_to(sender, DOWN)
        self.play(
            Transform(response_at_proxy, response_to_client)
        )
        self.wait(3)
