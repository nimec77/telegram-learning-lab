const assert=require("node:assert/strict");
const fs=require("node:fs");
const vm=require("node:vm");
const path=__dirname+"/";
const h=fs.readFileSync(path+"lesson-04.html","utf8");
const payload=h.match(/<script type="application\/json" id="sources">([\s\S]*?)<\/script>/)[1];
const sources=JSON.parse(payload);
for(const kind of ["starter","reference"]){
 assert.equal(sources[kind],fs.readFileSync(path+"lesson-04-"+kind+".rs","utf8"));
}
assert.equal(sources.starter.includes("todo!"),true);
assert.equal(sources.reference.includes("todo!"),false);
const ids=[...h.matchAll(/\bid="([^"]+)"/g)].map(m=>m[1]);
assert.equal(new Set(ids).size,ids.length);
for(const internal of h.matchAll(/href="#([^"]+)"/g))assert(ids.includes(internal[1]));
const elements=Object.fromEntries(ids.map(id=>[id,{textContent:"",focus(){},remove(){},click(){downloads.push([this.download,this.href])}}]));
const downloads=[],blobs=[];
elements.sources.textContent=payload;
const context=vm.createContext({
 document:{getElementById:id=>elements[id],createElement:()=>({click(){downloads.push([this.download,this.href])},remove(){}}),body:{append(){}}},
 URL:{createObjectURL:blob=>{blobs.push(blob);return "blob:fixture"},revokeObjectURL(){}},
 Blob, setTimeout:fn=>fn()
});
vm.runInContext(h.match(/<script>\n([\s\S]*?)<\/script>/)[1],context);
const cases=[
 ["","Empty"],[" \t\f\r\n","Empty"],[" /INSTALL\t","Draft(Install)"],["/update","Draft(Update)"],
 ["/watchlater","Draft(WatchLater)"],["/favorites","Draft(Favorites)"],["/install /update","Ambiguous"],
 ["/favorites send now","Ambiguous"],["/install@bot","Unknown"],["how do I update?","Unknown"],
 ["/help /install","Unknown"],["/install-more","Unknown"],["/install\u00a0","Unknown"],
 ["<script>alert(1)</script>","Unknown"],["/install\u000b","Unknown"]
];
for(const [input,want] of cases){
 elements.request.value=input; elements.run.onclick();
 assert(elements.out.textContent.includes(want),input);
 if(want.startsWith("Draft"))assert(elements.out.textContent.includes("Nothing was sent."));
 else assert(elements.out.textContent.includes("Nobody was notified."));
}
elements.mixed.onclick();assert.equal(elements.request.value,"/install /update");
elements.starter.onclick();elements.reference.onclick();
assert.deepEqual(downloads,[["lesson-04-starter.rs","blob:fixture"],["lesson-04-reference.rs","blob:fixture"]]);
Promise.all(blobs.map(b=>b.text())).then(got=>{
 assert.deepEqual(got,[sources.starter,sources.reference]);
 assert.equal(elements.referenceCode.textContent,sources.reference);
 console.log("PASS: 15 routing fixtures, both download payloads, reference display, quick input, unique IDs and navigation.");
});
