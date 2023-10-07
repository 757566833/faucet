import { existsSync, readdirSync, lstatSync, unlinkSync, rmdirSync } from 'fs';
import { join,dirname } from 'path';
import { fileURLToPath } from 'url';


const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);


function deleteFolderRecursive(folderPath) {
  if (existsSync(folderPath)) {
    if(lstatSync(folderPath).isDirectory()){
      readdirSync(folderPath).forEach((file) => {
        const curPath = join(folderPath, file);
  
        if (lstatSync(curPath).isDirectory()) {
          // 递归删除子文件夹
          deleteFolderRecursive(curPath);
        } else {
          // 删除文件
          unlinkSync(curPath);
        }
      });
  
      // 删除空文件夹
      rmdirSync(folderPath);
    }else{
      // 删除文件
      unlinkSync(folderPath);
    }
    
  }
}


const cache = [
  join(__dirname,'./.turbo'),
  join(__dirname,'./.yarn'),
  join(__dirname,'./apps/web/.next'),
  join(__dirname,'./apps/web/.turbo'),
  join(__dirname,'./apps/web/node_modules'),

  join(__dirname,'./.pnp.cjs'),
  join(__dirname,'./.pnp.loader.mjs'),
  
  join(__dirname,'./node_modules'),
  join(__dirname,'./target'),
]

for (const item of cache) {
  deleteFolderRecursive(item);
}