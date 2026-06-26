"""Integration tests — require running control-center (see enterprise_ops_smoke.sh)."""

import os
import unittest

from spanda_sdk import ControlCenterClient

BASE = os.environ.get("SPANDA_CONTROL_CENTER_URL", "http://127.0.0.1:8080")
SKIP = os.environ.get("SPANDA_SDK_INTEGRATION") != "1"


@unittest.skipUnless(not SKIP, "set SPANDA_SDK_INTEGRATION=1 with server running")
class IntegrationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.client = ControlCenterClient(base_url=BASE)

    def test_health(self) -> None:
        data = self.client.health()
        self.assertEqual(data.get("service"), "spanda-control-center")

    def test_rpc_dashboard(self) -> None:
        data = self.client.rpc("spanda.v1.SpandaService/GetDashboard")
        self.assertIn("result", data)


if __name__ == "__main__":
    unittest.main()
