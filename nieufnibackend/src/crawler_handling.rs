pub fn crawler_response(webpage_type: String, image: String, title: String, url: String, description: String) -> String {
    format!(r##"
<!DOCTYPE html>
<html lang="pl">
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, width=device-width, target-densitydpi=device-dpi" />
    <!--[if IE]><link rel="icon" href="/favicon.ico"><![endif]-->
    <link href="https://fonts.googleapis.com/css?family=VT323" rel="stylesheet">
    <meta property="og:type" content="{}">
    <meta property="og:image" content="{}">
    <meta property="og:title" content="{}">
    <meta property="og:url" content="{}">
    <meta property="og:description" content="{}">
    <meta property="fb:app_id" content="782857642061698">
    <title>{}</title>
  <link href="/articles.js" rel="prefetch"><link href="/contact.js" rel="prefetch"><link href="/redakcja.js" rel="prefetch"><link href="/app.js" rel="preload" as="script"><link rel="icon" type="image/png" sizes="32x32" href="/img/icons/favicon-32x32.png"><link rel="icon" type="image/png" sizes="16x16" href="/img/icons/favicon-16x16.png"><link rel="manifest" href="/manifest.json"><meta name="theme-color" content="#4DBA87"><meta name="apple-mobile-web-app-capable" content="no"><meta name="apple-mobile-web-app-status-bar-style" content="default"><meta name="apple-mobile-web-app-title" content="nieufnifront"><link rel="apple-touch-icon" href="/img/icons/apple-touch-icon-152x152.png"><link rel="mask-icon" href="/img/icons/safari-pinned-tab.svg" color="#4DBA87"><meta name="msapplication-TileImage" content="/img/icons/msapplication-icon-144x144.png"><meta name="msapplication-TileColor" content="#000000"></head>
  <body>
    <noscript>
      <strong>We're sorry but nieufni.pl doesn't work properly without JavaScript enabled. Please enable it to continue.</strong>
    </noscript>
    <div id="app"></div>
    <!-- built files will be auto injected -->
  <script type="text/javascript" src="/app.js"></script></body>
</html>
    "##, webpage_type, image, title, url, description, title)
}