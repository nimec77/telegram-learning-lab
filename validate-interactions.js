// Parse the real page before exercising its handlers. No fake DOM/value injection.
const fs = require('node:fs');
const assert = require('node:assert/strict');
const { JSDOM } = require('jsdom');
const dom = new JSDOM(fs.readFileSync('lesson-02.html', 'utf8'), {
  runScripts: 'dangerously', url: 'https://example.invalid/lesson-02.html'
});
const e = id => dom.window.document.getElementById(id);
const select = (id, value) => {
  const control = e(id);
  assert.ok([...control.options].some(o => o.value === value && !o.disabled), `${id}: selectable ${value} option required`);
  control.value = value;
  assert.equal(control.value, value);
  control.dispatchEvent(new dom.window.Event('change', { bubbles: true }));
};
const run = (result, calls) => {
  e('run').click();
  assert.equal(e('result').textContent, `${result}\nFake sender calls: ${calls}\nReal network calls: 0`);
};
assert.deepEqual([...e('mode').options].map(o => o.value), ['preview', 'dispatch']);
assert.deepEqual([...e('outcome').options].map(o => o.value), ['sent', 'refused', 'unknown']);
select('mode', 'preview'); run('Preview: no dispatch', 0);
select('mode', 'dispatch'); select('outcome', 'sent'); run('Sent: receipt 42', 1);
select('outcome', 'unknown'); run('Unknown: no automatic retry', 1);
run('Unknown: no automatic retry', 1); // Repeated clicks still make one attempt per run.
select('outcome', 'refused'); run('NotSent: definitive refusal', 1);
select('mode', 'preview'); run('Preview: no dispatch', 0);
e('enabled').checked = false; run('NotSent: disabled', 0);
e('reset').click(); assert.equal(e('mode').value, 'preview'); assert.equal(e('outcome').value, 'sent');
e('destination').value = 'other'; run('NotSent: destination', 0);
e('reset').click(); e('text').value = '   '; run('NotSent: text', 0);
e('reset').click(); e('emoji').click(); run('NotSent: text', 0);
e('reset').click(); e('text').value = '😀'.repeat(2048); run('Preview: no dispatch', 0);
dom.window.close();
console.log('PASS: parsed selectable modes and 11 UI state scenarios');
