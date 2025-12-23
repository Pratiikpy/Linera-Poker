const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Get Netlify auth token
let netlifyToken;
try {
  const configPath = path.join(process.env.APPDATA || process.env.HOME, 'netlify', 'config.json');
  const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
  netlifyToken = config.users?.[0]?.auth?.token || Object.values(config.users || {})[0]?.auth?.token;
} catch (e) {
  console.error('Error reading Netlify config:', e.message);
  process.exit(1);
}

if (!netlifyToken) {
  console.error('No Netlify auth token found');
  process.exit(1);
}

console.log('Found Netlify token, deploying...');

// Set token as environment variable
process.env.NETLIFY_AUTH_TOKEN = netlifyToken;

// Deploy using netlify-cli programmatically
try {
  const output = execSync('npx netlify-cli deploy --prod --dir=dist --site=linera-poker-conway --create-site --json', {
    cwd: __dirname,
    env: process.env,
    encoding: 'utf8'
  });

  const result = JSON.parse(output);
  console.log('\n✅ DEPLOYMENT SUCCESSFUL!');
  console.log('🌐 Live URL:', result.url || result.deploy_url);
  console.log('📝 Site ID:', result.site_id);
} catch (error) {
  console.error('Deployment failed:', error.message);
  process.exit(1);
}
