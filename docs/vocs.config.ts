import { defineConfig } from 'vocs'

const GITHUB_URL = 'https://github.com/christopherwxyz/remix-mcp'

export default defineConfig({
  title: 'remix-mcp',
  description: 'Control Ableton Live with AI via MCP',
  aiCta: false,
  logoUrl: { light: '/remix-light.svg', dark: '/remix-dark.svg' },
  iconUrl: '/favicon.svg',
  ogImageUrl: `${GITHUB_URL}/blob/main/banner.png?raw=true`,
  rootDir: '.',
  theme: {
    accentColor: '#007AFF',
    variables: {
      color: {
        background: { light: '#ffffff', dark: '#0a0a0f' },
        backgroundDark: { light: '#f8fafc', dark: '#111118' },
        text: { light: '#1a1a2e', dark: '#e4e4eb' },
        textSecondary: { light: '#64748b', dark: '#94a3b8' },
      },
      content: {
        horizontalPadding: '48px',
        verticalPadding: '80px',
      },
      fontFamily: {
        default: 'Inter, system-ui, -apple-system, sans-serif',
        mono: 'JetBrains Mono, ui-monospace, monospace',
      },
    },
  },
  topNav: [
    { text: 'Guide', link: '/getting-started', match: '/getting-started' },
    { text: 'Examples', link: '/examples', match: '/examples' },
    { text: 'Tools', link: '/tools', match: '/tools' },
    { text: 'Releases', link: `${GITHUB_URL}/releases` },
    { text: 'GitHub', link: GITHUB_URL },
  ],
  sidebar: [
    {
      text: 'Introduction',
      items: [
        { text: 'Getting started', link: '/getting-started' },
        { text: 'Installation', link: '/installation' },
        { text: 'Configuration', link: '/configuration' },
      ],
    },
    {
      text: 'Usage',
      items: [
        { text: 'Examples', link: '/examples' },
        { text: 'Troubleshooting', link: '/troubleshooting' },
      ],
    },
    {
      text: 'Tools reference',
      items: [
        { text: 'Overview', link: '/tools' },
        { text: 'Transport', link: '/tools/transport' },
        { text: 'Tracks', link: '/tools/tracks' },
        { text: 'Clips', link: '/tools/clips' },
        { text: 'Scenes', link: '/tools/scenes' },
        { text: 'Devices', link: '/tools/devices' },
        { text: 'Song', link: '/tools/song' },
        { text: 'Browser', link: '/tools/browser' },
        { text: 'View', link: '/tools/view' },
        { text: 'Cue points', link: '/tools/cue-points' },
      ],
    },
    {
      text: 'Development',
      items: [
        { text: 'Architecture', link: '/architecture' },
        { text: 'Contributing', link: '/contributing' },
        { text: 'Acknowledgements', link: '/acknowledgements' },
      ],
    },
  ],
  socials: [{ icon: 'github', link: GITHUB_URL }],
})
