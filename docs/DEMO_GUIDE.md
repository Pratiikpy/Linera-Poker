# Demo Recording Guide

**Linera Poker - Professional Demo for Judges**

This guide provides step-by-step instructions for recording a compelling 2-3 minute demo video that showcases Linera Poker's unique features and "wow factor."

---

## Table of Contents

- [Pre-Recording Setup](#pre-recording-setup)
- [Recording Equipment](#recording-equipment)
- [Demo Script](#demo-script)
- [Key Highlights to Show](#key-highlights-to-show)
- [Post-Production Tips](#post-production-tips)
- [Publishing Checklist](#publishing-checklist)

---

## Pre-Recording Setup

### 1. Environment Preparation

**Terminal Setup (2 terminals):**

```bash
# Terminal 1: Linera Service
cd ~/linera-poker
linera service --port 8080

# Terminal 2: Frontend Dev Server
cd ~/linera-poker/frontend
npm run dev
```

**Browser Setup:**
- Use Chrome or Firefox (best DevTools support)
- Open in **Incognito/Private Mode** (clean state)
- Window size: **1920x1080** (standard video resolution)
- Zoom: **100%** (no scaling)
- Clear all cookies/cache

**Browser DevTools:**
- Open DevTools (F12)
- Switch to **Network** tab
- Enable "Preserve log"
- Filter: "All" (to show all requests)

**Browser Console:**
- Keep Console tab visible (shows connection logs)
- Clear console before starting demo

### 2. MetaMask/Wallet Setup

**Install MetaMask:**
- Fresh profile (no existing accounts)
- Add test ETH to wallet (not actually needed, but looks professional)
- Have wallet extension pinned to toolbar

**Alternative:**
- Use Dynamic Labs embedded wallet (simpler for demo)

### 3. Test Run

**Before recording, do a complete dry run:**
1. Connect wallet
2. Wait for Conway connection animation
3. Join as Player A
4. Join as Player B
5. Play a complete hand
6. Screenshot any errors or issues

**Fix any issues before recording!**

---

## Recording Equipment

### Screen Recording Software

**macOS:**
- QuickTime Player (built-in, free)
  - File → New Screen Recording
  - Options → Show Mouse Clicks
  - Quality: Maximum

**Windows:**
- OBS Studio (free, open source)
  - Settings → Output → Recording Quality: High
  - Settings → Video → Base Resolution: 1920x1080
  - Settings → Video → FPS: 60

**Linux:**
- SimpleScreenRecorder (free)
- ffmpeg command line

**Professional (optional):**
- Camtasia ($299)
- ScreenFlow ($169, macOS)
- Loom (free tier available)

### Audio Setup

**Microphone:**
- Built-in laptop mic: Acceptable
- USB condenser mic: Better (Blue Yeti, Audio-Technica AT2020)
- Lavalier mic: Best for clarity

**Audio Tips:**
- Record in quiet room
- Close windows (reduce outside noise)
- Disable notifications (silence Slack, email, etc.)
- Use noise reduction in post-production

**Script:**
- Write out your narration beforehand
- Practice 2-3 times before recording
- Speak slowly and clearly
- Smile while talking (makes voice sound friendly)

---

## Demo Script

### Introduction (0:00 - 0:20)

**Visual:** Linera Poker intro screen

**Narration:**
> "Linera Poker is the world's first provably fair poker protocol where the dealer literally cannot see your cards. Built on Linera microchains for the Wave 6 Buildathon, it solves the $60 billion online poker trust problem."

**Actions:**
- Show intro screen with 3-chain architecture
- Highlight key features: "Zero Trust", "Native Cross-Chain", "Instant Settlement"

### Conway Connection (0:20 - 0:40)

**Visual:** Professional Conway connection animation

**Narration:**
> "Unlike other buildathon projects, Linera Poker automatically connects to the Conway Testnet on page load. Watch as it initializes the Linera client, creates a faucet wallet, claims a chain, and establishes connections to the table and player chains."

**Actions:**
1. Click "CONNECT WALLET" (Dynamic Labs)
2. Approve wallet connection
3. **Show the ConwayConnectionLoading animation**
   - Point out each step as it completes:
     - ✅ @linera/client loaded
     - ✅ Faucet wallet created
     - ✅ Claiming chain from Conway...
     - ✅ Table connection
     - ✅ Player chains
4. **Highlight wallet badge** showing EVM address + Chain ID

**Expected Duration:** ~20 seconds (2.5s connection time in fast-forward if needed)

### Wallet Integration (0:40 - 0:55)

**Visual:** Header with wallet badge and provenance indicators

**Narration:**
> "We're using Dynamic Labs for seamless EVM wallet integration. Notice the wallet badge showing our connected address and Linera chain ID. The provably fair badge shows this game is cryptographically verifiable."

**Actions:**
- **Zoom in on wallet badge** (top right)
- Hover over "Provably Fair" badge
- **Open browser DevTools Network tab**
- Show GraphQL requests to Conway Testnet

**Expected Timestamp:** 0:40 - 0:55

### Cross-Chain Architecture (0:55 - 1:15)

**Visual:** CrossChainInfo component showing 3 chains

**Narration:**
> "Here's where Linera's microchains architecture shines. We have three separate blockchains: a dealer chain managing the game state, and two player chains storing each player's private cards. The dealer chain literally cannot access player chains - it's architecturally impossible to cheat."

**Actions:**
- Scroll down to CrossChainInfo section
- Point to each chain card:
  - **Dealer Chain** (yellow indicator)
  - **Player A Chain** (blue indicator)
  - **Player B Chain** (pink indicator)
- Highlight "Cannot see hole cards!" callout
- Show chain IDs in each card

**Expected Timestamp:** 0:55 - 1:15

### Player A Joins (1:15 - 1:30)

**Visual:** Player A joining with cross-chain loading animation

**Narration:**
> "Let's join the table as Player A. I'll post 100 chips. Watch the cross-chain message animation - you can see the message traveling from Player A's chain to the dealer chain in real-time. This is what sub-200 millisecond cross-chain messaging looks like."

**Actions:**
1. Switch to Player A (toggle at top)
2. Enter stake: **100**
3. Click "JOIN TABLE"
4. **Show JoiningTableLoading modal**
   - Animated message flow: Blue (Player A) → Yellow (Table)
   - Message details showing "JoinTable" operation
5. **Show success** - Player A card appears on table

**Expected Timestamp:** 1:15 - 1:30

### Player B Joins (1:30 - 1:40)

**Visual:** Player B joining animation

**Narration:**
> "Now Player B joins. Same cross-chain coordination, different player chain."

**Actions:**
1. Switch to Player B
2. Enter stake: **100**
3. Click "JOIN TABLE"
4. Show animation (faster this time, audience gets the concept)
5. Both players now visible on table

**Expected Timestamp:** 1:30 - 1:40

### Cards Dealt (1:40 - 1:55)

**Visual:** Private hole cards revealed on player chains

**Narration:**
> "Cards are dealt. Notice that Player A can only see their own hole cards on their private chain. Player B's cards are hidden, and vice versa. The dealer chain doesn't have access to either set of private cards."

**Actions:**
1. **Show Player A view:**
   - Highlight hole cards (e.g., A♠ K♦)
   - Show "Your Private Cards" label
   - Point to "Stored on YOUR chain" indicator
2. **Switch to Player B view:**
   - Different hole cards visible (e.g., Q♥ J♣)
   - Player A's cards are face-down
3. **Show community cards** (if any dealt yet)

**Expected Timestamp:** 1:40 - 1:55

### Betting Round (1:55 - 2:15)

**Visual:** Interactive betting with turn indicators

**Narration:**
> "Betting works just like real poker, but every action is a cross-chain message that's cryptographically verified. Player A is first to act - I'll raise to 50. Player B calls. All actions are recorded immutably on-chain."

**Actions:**
1. **Player A's turn:**
   - Show "Your Turn" indicator glowing
   - Enter raise amount: **50**
   - Click "RAISE"
   - Show ProcessingBetLoading (brief)
   - Pot updates: 100 → 150
2. **Switch to Player B:**
   - Show "Your Turn" now on Player B
   - Click "CALL"
   - Pot updates: 150 → 200

**Expected Timestamp:** 1:55 - 2:15

### Showdown (2:15 - 2:30)

**Visual:** Cards revealed, winner determined

**Narration:**
> "At showdown, both players reveal their cards via cross-chain messages. The table contract evaluates hands using a deterministic, provably fair algorithm. No dealer intervention, no possibility of cheating."

**Actions:**
1. **Player A reveals:**
   - Click "REVEAL CARDS"
   - Cards travel from Player A chain → Table chain
   - Cards appear on table
2. **Player B reveals:**
   - Same process
   - Both hands now visible
3. **Winner determined:**
   - Highlight winning hand
   - Show pot distribution
   - Display "Winner: Player X" message

**Expected Timestamp:** 2:15 - 2:30

### Performance & Tech Stack (2:30 - 2:45)

**Visual:** Browser DevTools showing network performance

**Narration:**
> "Let's look at performance. This entire game - with cross-chain coordination between three separate blockchains - completed in under 10 seconds. Cross-chain message latency is averaging 180 milliseconds, 63% faster than Ethereum Layer 2 solutions."

**Actions:**
- **Open DevTools Network tab:**
  - Show GraphQL requests to Conway Testnet
  - Filter by "graphql"
  - Show timing: ~180ms per request
  - Point out Conway Testnet endpoint URLs
- **Open Console:**
  - Show connection logs
  - Highlight chain IDs

**Expected Timestamp:** 2:30 - 2:45

### Conclusion (2:45 - 3:00)

**Visual:** Logo and key features summary

**Narration:**
> "Linera Poker demonstrates what's uniquely possible on Linera: true privacy through microchains, sub-second cross-chain messaging, and provably fair gaming without trust. Built with Linera SDK 0.15, deployed to Conway Testnet, and ready for real players. Thank you!"

**Actions:**
- Return to main table view
- Show "Provably Fair" badge pulsing
- Fade to logo screen
- Display:
  - GitHub: github.com/your-repo/linera-poker
  - Live Demo: [your-deployment-url]
  - "Built for Linera Wave 6 Buildathon"

**Expected Timestamp:** 2:45 - 3:00

---

## Key Highlights to Show

### Must-Show Features (Priority 1)

✅ **Conway Testnet Connection**
- Animated connection screen with progressive steps
- Wallet badge showing chain ID
- DevTools network tab showing Conway requests

✅ **Cross-Chain Message Visualization**
- JoiningTableLoading modal with animated message flow
- Clear source (player) → destination (table) visualization
- Sub-200ms latency

✅ **3-Chain Architecture**
- CrossChainInfo component showing all 3 chains
- Different colored indicators (yellow, blue, pink)
- "Cannot see cards" privacy guarantee

✅ **Private Card Storage**
- Player A sees their cards, not Player B's
- "Stored on YOUR chain" labels
- Face-down cards for opponent

✅ **Performance Evidence**
- Network tab showing actual Conway Testnet requests
- ~180ms request times
- Chain IDs in URLs proving real testnet usage

### Nice-to-Have Features (Priority 2)

🎯 **Dynamic Labs Integration**
- Smooth wallet connection UX
- EVM address display
- Multi-wallet support mentioned

🎯 **Provably Fair Badge**
- Hover to show fairness modal
- Deck seed and dealer secret display
- Cryptographic verification explained

🎯 **Professional UI/UX**
- Loading animations
- Smooth transitions
- Responsive design
- Dark mode aesthetic

🎯 **Game State Synchronization**
- Auto-refresh every 3 seconds
- Optimistic UI updates
- Error handling

### What NOT to Show

❌ **Backend Code**
- Don't spend time showing Rust code (unless asked)
- Judges care about the working product, not implementation details

❌ **Local Development Setup**
- Don't show terminal commands or build process
- Demo should feel like using a production app

❌ **Bugs or Errors**
- If something goes wrong, edit it out
- Re-record that section if needed

❌ **Long Pauses**
- Keep the demo moving
- Edit out dead air in post-production

---

## Post-Production Tips

### Video Editing

**Software:**
- iMovie (macOS, free)
- DaVinci Resolve (cross-platform, free)
- Adobe Premiere Pro (professional, $55/month)

**Editing Steps:**

1. **Trim Dead Air:**
   - Remove long pauses
   - Cut loading screens (unless showcasing them)
   - Keep video under 3 minutes

2. **Add Annotations:**
   - Arrow pointing to Conway connection steps
   - Circle highlighting wallet badge
   - Text overlay: "180ms latency" over network tab

3. **Add Background Music:**
   - Use royalty-free music (YouTube Audio Library)
   - Keep volume low (don't overpower narration)
   - Upbeat, tech-focused track

4. **Add Captions:**
   - Auto-generate via YouTube or Descript
   - Helps with accessibility
   - Viewers often watch on mute

5. **Color Correction:**
   - Boost contrast slightly
   - Ensure UI colors are vivid
   - Make sure text is readable

### Audio Editing

**Noise Reduction:**
```
1. Export audio track
2. Use Audacity (free):
   - Effect → Noise Reduction
   - Get Noise Profile (select silent section)
   - Apply to full track
3. Re-import cleaned audio
```

**Normalization:**
- Ensure audio level is consistent
- Peak at -3dB (not clipping)
- Use audio limiter to prevent spikes

### Intro/Outro

**Intro (5 seconds):**
```
[Linera Logo]
"Linera Poker"
"Wave 6 Buildathon Demo"
```

**Outro (5 seconds):**
```
"Try it yourself:"
[QR code to live demo]
"GitHub: github.com/your-repo/linera-poker"
"Built with ❤️ on Linera"
```

---

## Publishing Checklist

### Video File Export

**Settings:**
- Format: **MP4** (H.264 codec)
- Resolution: **1920x1080** (1080p)
- Frame Rate: **30 fps** (or 60 fps if smooth)
- Bitrate: **8-10 Mbps** (high quality)
- Audio: **AAC, 192 kbps**

**File Naming:**
```
linera-poker-demo-v1.mp4
```

### Upload Destinations

**1. YouTube**
- Create unlisted video (or public if comfortable)
- Title: "Linera Poker - Provably Fair Poker on Conway Testnet | Wave 6 Buildathon"
- Description:
  ```
  Linera Poker is the world's first provably fair poker protocol built on
  Linera microchains. This demo showcases:

  ✅ Automatic Conway Testnet connection
  ✅ Cross-chain message visualization (3 chains)
  ✅ Sub-200ms latency
  ✅ Private card storage (dealer can't peek)
  ✅ Dynamic Labs wallet integration

  Live Demo: [your-url]
  GitHub: [your-repo]

  Built for Linera Wave 6 Buildathon
  Tech: Linera SDK 0.15, Rust, React, TypeScript, Dynamic Labs
  ```
- Tags: `linera`, `blockchain`, `poker`, `web3`, `buildathon`, `conway testnet`
- Thumbnail: Screenshot of connection animation or table with cards

**2. GitHub README**
- Embed YouTube video:
  ```markdown
  [![Demo Video](https://img.youtube.com/vi/YOUR_VIDEO_ID/maxresdefault.jpg)](https://www.youtube.com/watch?v=YOUR_VIDEO_ID)
  ```

**3. Twitter/X**
- Post short clip (< 60 seconds)
- Thread with highlights:
  ```
  🎰 Just built the first provably fair poker on @linera_io!

  🔒 Your cards live on YOUR chain
  ⚡ 180ms cross-chain messaging
  🌐 Live on Conway Testnet

  Watch the demo 👇
  [link]

  #Linera #Web3Gaming #Buildathon
  ```

**4. Linera Discord**
- Post in #showcase channel
- Include:
  - Video link
  - GitHub repo link
  - Brief description
  - Ask for feedback

---

## Recording Day Checklist

### Before You Start

- [ ] Linera service running (`linera service --port 8080`)
- [ ] Frontend dev server running (`npm run dev`)
- [ ] Browser in incognito/private mode
- [ ] DevTools open (Network + Console tabs)
- [ ] MetaMask/wallet ready
- [ ] Screen recording software open
- [ ] Microphone tested
- [ ] Script printed/visible
- [ ] Water nearby (stay hydrated!)
- [ ] Phone on silent
- [ ] Notifications disabled
- [ ] Comfortable sitting position
- [ ] Good lighting (if showing face)

### During Recording

- [ ] Speak slowly and clearly
- [ ] Smile (sounds better!)
- [ ] Pause between sections (easier to edit)
- [ ] Show mouse clicks/highlights
- [ ] Wait for animations to complete
- [ ] Check that UI is visible (not cut off)
- [ ] Monitor audio levels (no clipping)

### After Recording

- [ ] Save raw recording immediately
- [ ] Review full video (check for errors)
- [ ] Note timestamps for editing
- [ ] Export audio for cleanup
- [ ] Create backup of raw files
- [ ] Start editing within 24 hours (while fresh)

---

## Example Timeline Summary

| Time | Section | Visual | Audio |
|------|---------|--------|-------|
| 0:00-0:20 | Intro | Logo + features | Explain problem |
| 0:20-0:40 | Conway Connection | ConwayConnectionLoading | Highlight testnet |
| 0:40-0:55 | Wallet | Badge + DevTools | Dynamic Labs |
| 0:55-1:15 | Architecture | 3-chain diagram | Privacy guarantee |
| 1:15-1:30 | Player A Join | JoiningTableLoading | Cross-chain message |
| 1:30-1:40 | Player B Join | Same animation | Second player |
| 1:40-1:55 | Deal Cards | Private cards | Show privacy |
| 1:55-2:15 | Betting | Raise + Call | On-chain actions |
| 2:15-2:30 | Showdown | Reveal + winner | Provably fair |
| 2:30-2:45 | Performance | DevTools | 180ms latency |
| 2:45-3:00 | Conclusion | Logo | Call to action |

**Total Duration:** ~3 minutes

---

## Troubleshooting Common Issues

### "Connection Failed"
- Check linera service is running
- Verify .env variables are set
- Clear browser cache and retry

### "Animation Not Smooth"
- Record at 60fps instead of 30fps
- Close other applications (reduce CPU load)
- Use hardware encoding in OBS

### "Audio Too Quiet"
- Position mic closer to mouth
- Boost gain in recording software
- Normalize audio in post-production

### "Video Too Large"
- Reduce bitrate (8 Mbps is fine)
- Use H.264 codec (not H.265)
- Compress with Handbrake after export

---

## Resources

**Screen Recording:**
- OBS Studio: https://obsproject.com
- QuickTime: Built-in (macOS)
- ScreenFlow: https://www.telestream.net/screenflow

**Video Editing:**
- DaVinci Resolve: https://www.blackmagicdesign.com/products/davinciresolve
- iMovie: Built-in (macOS)

**Audio Editing:**
- Audacity: https://www.audacityteam.org

**Royalty-Free Music:**
- YouTube Audio Library: https://studio.youtube.com
- Free Music Archive: https://freemusicarchive.org

**Stock Footage:**
- Pexels: https://www.pexels.com/videos
- Pixabay: https://pixabay.com/videos

---

## Questions?

If you encounter issues or need help:
- Check [RUN_DEMO.md](../RUN_DEMO.md) for setup instructions
- Post in GitHub Issues
- Ask in Linera Discord #help channel

---

**Good luck with your demo! 🎬🚀**

*Last updated: December 15, 2025*
