const data =  {
        "ipo": [],
        "job_postings": [],
         "mergers_and_acquisitions": ["fsi sandbox", "kaffeine"],
        "product_launch": ["fsi sandbox", "kaffeine"]
    }

    function isEmptyObject(data) {
     if (Array.isArray(data)) {
       return data.length === 0;
     }
     return Object.keys(data).length === 0;
   }
 const terms = []
 for (const single of Object.keys(data)) {
  if (isEmptyObject(data[single])) {
   const sanitized = single.replace(/[/[`~!@#$%^&*()_|+\-=?;:'",.<>{}[\]\\/]/gi, ' ');
   terms.push(sanitized)
  } else {
   terms.push(data[single].join(', '))
  }
 }

 console.log(terms);