# This file is part of DarkFi (https://dark.fi)
#
# Copyright (C) 2020-2023 Dyne.org foundation
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

import urwid
import logging
import asyncio

from scroll import ScrollBar, Scrollable
from model import Model

event_loop = asyncio.get_event_loop()

class LeftList(urwid.ListBox):
    def focus_next(self):
        try: 
            self.body.set_focus(self.body.get_next(self.body.get_focus()[1])[1])
        except:
            pass
    def focus_previous(self):
        try: 
            self.body.set_focus(self.body.get_prev(self.body.get_focus()[1])[1])
        except:
            pass            

    def load_info(self):
        return InfoWidget(self)

class NodeView(urwid.WidgetWrap):
    def __init__(self, info):
        self.name = info
        self.text = urwid.Text(f"{self.name}")
        super().__init__(self.text)
        self._w = urwid.AttrWrap(self._w, None)
        self.update_w()

    def selectable(self):
        return True

    def keypress(self, size, key):
        #if key in ('q'):
        #    raise urwid.ExitMainLoop()
        return key

    def update_w(self):
        self._w.focus_attr = 'line'

    def get_widget(self):
        return "NodeView"

    def get_name(self):
        return self.name

class ConnectView(urwid.WidgetWrap):
    def __init__(self, info):
        self.name = info
        self.text = urwid.Text(f"{self.name}")
        super().__init__(self.text)
        self._w = urwid.AttrWrap(self._w, None)
        self.update_w()

    def selectable(self):
        return True

    def keypress(self, size, key):
        #if key in ('q'):
        #    raise urwid.ExitMainLoop()
        return key

    def update_w(self):
        self._w.focus_attr = 'line'

    def name(self):
        return "ConnectView"

    def get_name(self):
        return self.name

class SlotView(urwid.WidgetWrap):
    def __init__(self, info):
        self.name = info
        self.text = urwid.Text(f"{self.name}")
        super().__init__(self.text)
        self._w = urwid.AttrWrap(self._w, None)
        self.update_w()

    def selectable(self):
        return True

    def keypress(self, size, key):
        #if key in ('q'):
        #    raise urwid.ExitMainLoop()
        return key

    def update_w(self):
        self._w.focus_attr = 'line'

    def name(self):
        return "SlotView"

    def get_name(self):
        return self.name

class View():
    palette = [
              ('body','light gray','black', 'standout'),
              ("line","dark cyan","black","standout"),
              ]

    def __init__(self, data):
        #logging.debug(f"dnetview init {data}")

        self.data = data
        info_text = urwid.Text("")
        self.pile = urwid.Pile([info_text])
        scroll = ScrollBar(Scrollable(self.pile))
        rightbox = urwid.LineBox(scroll)
        
        #self.node_info = urwid.Text("")
        #widget = NodeView(self.node_info)

        #self.connect_info = urwid.Text("")
        #widget2 = ConnectView(self.connect_info)

        #self.slot_info = urwid.Text("")
        #widget3 = SlotView(self.slot_info)

        self.listbox_content = []
        self.listwalker = urwid.SimpleListWalker(self.listbox_content)
        self.list = LeftList(self.listwalker)
        leftbox = urwid.LineBox(self.list)

        columns = urwid.Columns([leftbox, rightbox], focus_column=0)
        self.ui = urwid.Frame(urwid.AttrWrap( columns, 'body' ))

    #def update_view(self, loop, data):
    #    data = self.data

    #    logging.debug(len(data.nodes))
    #    for name, values in data.nodes.items():
    #        self.node_info = urwid.Text(name)
    #        widget = NodeView(self.node_info)
    #        self.listbox_content.append(widget)

    #    #loop.set_alarm_in(2, update_view)
    #    #await asyncio.sleep(0.1)

    async def update_view(self, data):
        while True:
            names = []
            for item in self.listwalker.contents:
                name = item.get_name()
                names.append(name)

            for name, values in data.nodes.items():
                if name in names:
                    continue

                else:
                    widget = NodeView(name)
                    self.listwalker.contents.append(widget)

                    info = values["result"]
                    channels = info["channels"]
                    channel_lookup = {}
                    for channel in channels:
                        id = channel["id"]
                        channel_lookup[id] = channel

                    for channel in channels:
                        if channel["session"] != "inbound":
                            continue
                        widget = ConnectView("inbound")
                        self.listwalker.contents.append(widget)

                        url = channel["url"]
                        widget = SlotView(url)
                        self.listwalker.contents.append(widget)

                    widget = ConnectView("  outbound")
                    self.listwalker.contents.append(widget)
                    for i, id in enumerate(info["outbound_slots"]):
                        if id == 0:
                            widget = SlotView(f"  {i}: none")
                            self.listwalker.contents.append(widget)
                            continue

                        assert id in channel_lookup
                        url = channel_lookup[id]["url"]
                        widget = SlotView(f"  {i}: {url}")
                        self.listwalker.contents.append(widget)

                    for channel in channels:
                        if channel["session"] != "seed":
                            continue
                        widget = ConnectView("seed")
                        self.listwalker.contents.append(widget)

                        url = channel["url"]
                        widget = SlotView(f"  {url}")
                        self.listwalker.contents.append(widget)

                    for channel in channels:
                        if channel["session"] != "manual":
                            continue
                        widget = ConnectView("manual")
                        self.listwalker.contents.append(widget)

                        url = channel["url"]
                        widget = SlotView(url)
                        self.listwalker.contents.append(widget)

                        print(f"  {url}")

            await asyncio.sleep(0.1)

    async def render_info(self, channels):
        while True:
            await asyncio.sleep(0.1)
            self.pile.contents.clear()
            focus_w = self.listbox.get_focus()
            match focus_w[0].name():
                case "NodeView":
                    self.pile.contents.append((urwid.Text(f""), self.pile.options()))
                case "ConnectView":
                    self.pile.contents.append((urwid.Text("2"), self.pile.options()))
                case "SlotView":
                    self.pile.contents.append((urwid.Text("3"), self.pile.options()))
