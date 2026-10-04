// Deterministic read-only analysis. Grid boundaries do not wrap.
const mean = a => a.reduce((s,x)=>s+x,0)/a.length
export function correlation(a,b) {
  if (a.length !== b.length || !a.length || [...a,...b].some(v=>typeof v!=='number'||!Number.isFinite(v))) return null
  const x=mean(a),y=mean(b)
  let aa=0,bb=0,ab=0
  for(let i=0;i<a.length;i++) { const u=a[i]-x,v=b[i]-y; aa+=u*u;bb+=v*v;ab+=u*v }
  return aa>0&&bb>0?ab/Math.sqrt(aa*bb):null
}
export function fieldPatches(values,side,fraction,cellSize=16) {
  if(values.length!==side*side||side<1||![0.25,0.5].includes(fraction)||values.some(x=>!Number.isSafeInteger(x)||x<0)) throw Error('Invalid field geometry')
  const threshold=[...values].sort((a,b)=>b-a)[Math.ceil(values.length*fraction)-1]
  const mask=values.map(v=>v>0&&v>=threshold), labels=Array(values.length).fill(-1),components=[]
  for(let i=0;i<mask.length;i++) {
    if(!mask[i]||labels[i]>=0) continue
    const id=components.length,queue=[i],cells=[]; labels[i]=id
    let perimeter=0,x=0,y=0
    for(let q=0;q<queue.length;q++) {
      const k=queue[q],cx=k%side,cy=Math.floor(k/side);cells.push(k);x+=cx+0.5;y+=cy+0.5
      for(const [nx,ny] of [[cx-1,cy],[cx+1,cy],[cx,cy-1],[cx,cy+1]]) {
        if(nx<0||ny<0||nx>=side||ny>=side||!mask[ny*side+nx]) { perimeter++;continue }
        const n=ny*side+nx;if(labels[n]<0) { labels[n]=id;queue.push(n) }
      }
    }
    components.push({id,cells,area:cells.length*cellSize**2,perimeter:perimeter*cellSize,
      perimeter_area:perimeter/(cells.length*cellSize),centroid:[x/cells.length*cellSize,y/cells.length*cellSize]})
  }
  const lags=[]
  for(let lag=1;lag<=Math.min(8,side-1);lag++) {
    const a=[],b=[]
    for(let y=0;y<side;y++) for(let x=0;x<side;x++) {
      const i=y*side+x
      if(x+lag<side){a.push(values[i]);b.push(values[i+lag])}
      if(y+lag<side){a.push(values[i]);b.push(values[i+lag*side])}
    }
    lags.push({distance:lag*cellSize,correlation:correlation(a,b)})
  }
  const crossed=lags.find(v=>v.correlation!==null&&v.correlation<=Math.exp(-1))
  const rich=mask.flatMap((v,i)=>v?[i]:[]),nearest=[]
  for(const i of rich) {
    let best=Infinity
    for(const j of rich) if(i!==j) {
      const d=(i%side-j%side)**2+(Math.floor(i/side)-Math.floor(j/side))**2
      best=Math.min(best,d);if(best===1)break
    }
    if(Number.isFinite(best))nearest.push(Math.sqrt(best)*cellSize)
  }
  return {threshold,rich_cells:rich.length,mask,labels,components,lags,
    correlation_length:crossed?.distance??null,
    correlation_length_censored:!crossed&&lags.some(v=>v.correlation!==null),
    max_measured_lag:lags.at(-1)?.distance??0,
    nearest_same_rich_mean:nearest.length?mean(nearest):null}
}
export function patchPersistence(previous,current) {
  if(previous.mask.length!==current.mask.length) throw Error('Patch geometry mismatch')
  let intersection=0,union=0
  const counts=new Map()
  for(let i=0;i<current.mask.length;i++) {
    if(previous.mask[i]||current.mask[i])union++
    if(previous.mask[i]&&current.mask[i]) {
      intersection++
      const key=`${previous.labels[i]},${current.labels[i]}`;counts.set(key,(counts.get(key)??0)+1)
    }
  }
  // Independent best overlap per prior patch; mergers/splits remain observable.
  const matches=previous.components.map(a=> {
    let best=null
    for(const b of current.components) {
      const overlap=counts.get(`${a.id},${b.id}`)??0
      const jaccard=overlap/(a.cells.length+b.cells.length-overlap)
      if(overlap&&(!best||jaccard>best.jaccard)) best={previous:a.id,current:b.id,jaccard,
        centroid_movement:Math.hypot(a.centroid[0]-b.centroid[0],a.centroid[1]-b.centroid[1]),area_ratio:b.area/a.area}
    }
    return best??{previous:a.id,current:null,jaccard:0,centroid_movement:null,area_ratio:null}
  })
  return {mask_jaccard:union?intersection/union:null,matches}
}
export function resourceCoherence(snapshots) {
  const previous=new Map()
  return snapshots.map(s=> {
    const fields=[]
    for(const channel of ['soft','hard','soft_productivity','hard_productivity']) for(const fraction of [0.25,0.5]) {
      const p=fieldPatches(s[channel],s.side,fraction,s.cell_size),key=`${channel}/${fraction}`
      const persistence=previous.has(key)?patchPersistence(previous.get(key),p):null
      previous.set(key,p)
      const {mask,labels,components,...rest}=p
      fields.push({channel,fraction,...rest,components:components.map(({cells,...p})=>p),persistence})
    }
    const overlaps=[0.25,0.5].map(fraction=> {
      const a=previous.get(`soft/${fraction}`).mask,b=previous.get(`hard/${fraction}`).mask
      let intersection=0,union=0;for(let i=0;i<a.length;i++){intersection+=Number(a[i]&&b[i]);union+=Number(a[i]||b[i])}
      return {fraction,intersection_cells:intersection,jaccard:union?intersection/union:null}
    })
    return {tick:s.tick,fields,soft_hard_overlap:overlaps}
  })
}
