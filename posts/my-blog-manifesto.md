# My Blog Manifesto.
<time>Aug 7, 2019</time>

I'm frustrated with the web as it stands today. Instead of delivering content, it seems instead that every site you visit tries to make you the product. Invasive advertising fingerprints my every move. Autoplaying videos load when all I want to do is _read the darn article_, and popups, redirects and suffocatingly large persistent headers and footers make it impossible to get to the content.

Well, I'm tired of it. And, as [others have done](https://sheep.horse/2016/6/a_website_manifesto_-_introducing_sheep.horse.html), I'm going to take a stand, small though it may be. I'm going to do my best to show that a website can serve it's purpose, without the bloat, the kruft, or the distractions.

I wanted to prove that web technologies could do better. And, somewhat counterintuitively, I think that I can do it best by throwing most of those technologies away.

Don't believe me?

Here's a manifesto to prove it.

---

#### My manifesto, a list of qualities I believe any good website should have.

- It should load fast.
- If it's about the content, then it should place the content front and center.
- Colors should be easy on the eyes
- Consistent color themeing
- No external javascript. You alone should control your user experience, not a larger corporation with different priorities.
- No references to third party resources, whenever possible. Social media buttons, comment sections- forget 'em.
- Pages should look good on any device.
- No sticky headers, autoplaying videos, etc.
- The source code should be readable- you shouldn't even need to minify your HTML.
- **Always HTTPS.** It's 2019, you're serving static pages. There's no excuse for HTTP.
- It should load super fast. It's a blog, not a 4K movie.
- No gimmicky animations. Leave them for the middle school powerpoints.
- No cookies or tracking tokens. Be GDPR compliant, _without_ the cookie banner.

I do my best to embody these goals with this blog. Everything is a static HTML page, that's currently hosted on [Netlify's free tier](https://www.netlify.com), but that can just as easily be moved to any other platform. On the other end, there's a bunch of markdown files, and a Rust script I built to autogenerate the HTML.

I use Rust because it's fast, and I wanted the chance to evaluate it's package ecosystem as a whole. It even does syntax highlighting for code in pure HTML, so I don't need third party JS libraries to render my code.

For look and feel, the website is built more or less from scratch, but I heavily referenced [Water.css](https://watercss.netlify.com/) and [58 bytes of css to look great nearly everywhere](https://jrl.ninja/etc/1/). I also used [coolors](https://coolors.co) for creating my color palatte.

I'm going to do my best to avoid editing the theme of the blog from now on. I'm always impressed by the timelessness of blogs like [The Daring Fireball](https://daringfireball.net/), so I will do my best to emulate it.

Source code for the static site generator and the blog as a whole is available at my github [here](https://github.com/arya-k/blog/).
