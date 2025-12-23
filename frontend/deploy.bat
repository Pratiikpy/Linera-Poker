@echo off
echo.
echo ========================================
echo   Linera Poker - Netlify Deployment
echo ========================================
echo.
echo This will deploy your poker game to Netlify.
echo.
echo When prompted:
echo   1. Press ENTER (to link to existing project)
echo   2. Press ENTER again (to use GitHub repo)
echo   3. Wait for deployment...
echo.
pause
echo.
cd /d "%~dp0"
npx netlify-cli deploy --prod --dir=dist
echo.
echo ========================================
echo   Deployment Complete!
echo ========================================
pause
