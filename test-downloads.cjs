const assert=require('node:assert/strict'),fs=require('node:fs'),{JSDOM}=require('jsdom');
const html=fs.readFileSync(__dirname+'/lesson-03.html','utf8');
let blob,filename,printed=false;
const dom=new JSDOM(html,{runScripts:'dangerously',url:'https://example.invalid',beforeParse(w){
w.URL.createObjectURL=b=>(blob=b,'blob:test');w.URL.revokeObjectURL=()=>{};
w.Blob=class {constructor(parts){this.text=parts.join('')}};w.print=()=>printed=true;
w.HTMLAnchorElement.prototype.click=function(){filename=this.download};
}});
const e=id=>dom.window.document.getElementById(id);
e('notes').value='SYNTHETIC_PRIVATE_NOTE';e('notes').dispatchEvent(new dom.window.Event('input'));
e('starter').click();assert.equal(filename,'lesson-03-starter.rs');assert.equal(blob.text,fs.readFileSync(__dirname+'/lesson-03-starter.rs','utf8'));
e('reference').click();assert.equal(blob.text,fs.readFileSync(__dirname+'/lesson-03-reference.rs','utf8'));
e('exportNotes').click();assert.equal(blob.text,'SYNTHETIC_PRIVATE_NOTE');
e('download').click();assert.ok(!blob.text.includes('SYNTHETIC_PRIVATE_NOTE'));assert.match(blob.text,/<html lang="en">/);
const copy=new JSDOM(blob.text,{runScripts:'dangerously',url:'https://example.invalid'});copy.window.document.getElementById('run').click();assert.match(copy.window.document.getElementById('result').textContent,/Ready/);copy.window.close();
e('print').click();assert.ok(printed);
const c=dom.window.document.querySelector('[data-progress]');c.click();assert.equal(e('progressText').textContent,'1 of 4 complete');e('resetProgress').click();assert.equal(e('progressText').textContent,'0 of 4 complete');assert.equal(e('notes').value,'SYNTHETIC_PRIVATE_NOTE');dom.window.close();
const offline=new JSDOM(html,{runScripts:'dangerously',url:'file:///lesson-03.html'});const n=offline.window.document.getElementById('notes');n.value='temporary';n.dispatchEvent(new offline.window.Event('input'));assert.match(offline.window.document.getElementById('saveStatus').textContent,/unavailable/);offline.window.document.getElementById('run').click();assert.match(offline.window.document.getElementById('result').textContent,/Ready/);offline.window.close();
console.log('PASS: all downloads, clean exported HTML reopens, print, checklist/reset, storage-unavailable offline mode');
