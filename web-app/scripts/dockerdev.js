const { execSync } = require('child_process');

process.env.PORT = "8080";
execSync("npm run dev", {stdio: [0,1,2], shell:true , uid: parseInt(process.env.SETUID), gid: parseInt(process.env.SETGID)});
