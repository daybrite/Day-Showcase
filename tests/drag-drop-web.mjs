// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0
// DAY_WEB_DRIVER adapter: navigate with dayscript/drag-drop.yaml. The screenshot
// request exercises real Chromium mouse input, then returns the resulting page.
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import http from 'node:http';
import path from 'node:path';
const [url, port] = process.argv.slice(2);
const root = process.env.DAY_WEB_DRIVER_PLAYWRIGHT || process.cwd();
const { chromium } = createRequire(path.join(root, 'resolve.js'))('playwright');
const browser = await chromium.launch({
  ...(process.env.DAY_DND_CHROMIUM ? { executablePath: process.env.DAY_DND_CHROMIUM } : {}),
});
const page = await browser.newPage({ viewport: { width: 1000, height: 900 } });
await page.goto(url);
let tested = false;
async function test() {
  const zones = page.locator('[draggable="true"]');
  await zones.nth(5).waitFor();
  const text = i => zones.nth(i).innerText();
  async function drag(from, to, rejected = false) {
    const a = await zones.nth(from).boundingBox(), b = await zones.nth(to).boundingBox();
    await page.mouse.move(a.x + a.width / 2, a.y + 40);
    await page.mouse.down();
    await page.mouse.move(a.x + a.width / 2 + 12, a.y + 40, { steps: 4 });
    await page.mouse.move(b.x + (rejected ? 8 : b.width / 2), b.y + 40, { steps: 20 });
    await page.mouse.up();
    await page.waitForTimeout(300);
  }
  assert.equal(await text(0), 'image/png');
  // The location guard must reject even when the type is permitted.
  await drag(0, 3, true);
  assert.equal(await text(0), 'image/png');
  assert.equal(await text(3), 'Drop here');
  await drag(0, 3);
  assert.equal(await text(0), 'Drop here');
  assert.equal(await text(3), 'image/png');
  // Binary custom data (including NUL and 0xff) travels through DataTransfer.
  await drag(2, 5);
  assert.equal(await text(2), 'Drop here');
  assert.equal(await text(5), 'application/vnd.day.showcase-card');
  assert.match((await page.locator('body').innerText()).replace(/[\u2066-\u2069]/g, ''), /5 bytes/);
  console.log('Native HTML drag: rejected region, image move, custom binary move passed');
}
const server = http.createServer(async (req, res) => {
  if (req.url === '/quit') {
    res.end('bye'); server.close(); await browser.close(); return;
  }
  if (req.url !== '/screenshot') { res.writeHead(404); res.end(); return; }
  try {
    if (!tested) { await test(); tested = true; }
    const png = await page.screenshot();
    res.writeHead(200, { 'Content-Type': 'image/png', 'Content-Length': png.length });
    res.end(png);
  } catch (error) {
    console.error(error); res.writeHead(500); res.end(String(error));
  }
});
server.listen(Number(port), '127.0.0.1');
