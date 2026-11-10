html, body {
  margin: 0;
  height: 100vh; 
  height: 100dvh; 
  overflow: hidden; 
}
body {
  display: flex;
  flex-direction: column;
  background-color: #333333;
}
header {
  text-align: center;
  font-family: system-ui, -apple-system, sans-serif;
  margin-top: 10px;
}
header h1 {
  margin: 0;
  color: #A0C4FF;
  font-weight: 700;
  font-size: 2rem;
  text-shadow: -1px -1px 0 #09192e, 1px -1px 0 #09192e, -1px  1px 0 #09192e, 1px  1px 0 #09192e;
}
header .subtitle {
  margin: 5px 0 0 0;
  color: #709CE6;
  font-weight: 500;
  font-size: 1.1rem; 
  letter-spacing: 0.05em;
  #text-transform: uppercase;
  text-shadow: -1px -1px 0 #09192e, 1px -1px 0 #09192e, -1px  1px 0 #09192e, 1px  1px 0 #09192e;
}
nav {
  width: 90%;
  margin-left: auto;
  margin-right: auto;
  color: #A0C4FF;
  font-weight: 700;
  font-family: system-ui, -apple-system, sans-serif;
  text-shadow: -1px -1px 0 #09192e, 1px -1px 0 #09192e, -1px  1px 0 #09192e, 1px  1px 0 #09192e;
  a { color: #00E5FF; }
}
main, .content {
  width: 90%;
  margin-left: auto;
  margin-right: auto;
  display: flex;
  flex-direction: column;
  flex: 1;
  border-radius: 16px; 
  border: 4px groove rgba(255, 255, 255, 0.2); 
  outline: 4px ridge #031627;
  outline-offset: 0px;
  color: #A0C4FF;
  font-weight: 700;
  font-family: system-ui, -apple-system, sans-serif;
  text-shadow: -1px -1px 0 #09192e, 1px -1px 0 #09192e, -1px  1px 0 #09192e, 1px  1px 0 #09192e; 
  display: block; 
  padding: 20px;
  box-sizing: border-box;
  margin-top: 20px;
  margin-bottom: 20px;
  overflow-y: auto;
  min-height: 0;
  position: relative;
}
footer, .footer {
  width: 90%;
  margin-left: auto;
  margin-right: auto;
  color: #A0C4FF;
  font-weight: 700;
  font-family: system-ui, -apple-system, sans-serif;
  text-shadow: -1px -1px 0 #09192e, 1px -1px 0 #09192e, -1px  1px 0 #09192e, 1px  1px 0 #09192e;
  padding-bottom: 10px;
}

.tabs {
  display: flex;
  flex-wrap: wrap;
  width: 90%;
  margin: 0 auto;
}
.tabs input[type="radio"] {
  display: none;
}
.tab-label {
  padding: 10px 20px;
  background: #333333;
  cursor: pointer;
  order: 1;
  border-top-left-radius: 4px;
  border-top-right-radius: 4px;
  margin-right: 2px;
}
.tab-content {
  position: absolute;
  top: 5%;
  left: 5%;
  width: 90%;
  height: 90%;
  box-sizing: border-box;
  overflow-y: auto;
  min-height: 0;
  padding: 20px;
  background: url('{{WATERMARK}}') no-repeat center center fixed, rgba(0, 0, 0, 0.2);
  background-size: 400px auto;
  background-attachment: local;
  border: 1px solid #ccc;
  display: none; 
  margin-left: auto;
  margin-right: auto;
  margin-top: 25px;
  margin-bottom: 10px;
}
.tabs input[type="radio"]:checked + .tab-label + .tab-content {
  display: block;
}
.tabs input[type="radio"]:checked + .tab-label {
  background: #000000;
  border: 1px solid #CCCCCC;
  border-bottom: 1px solid transparent;
  position: relative;
  z-index: 10;
}    
textarea {
  background-color: transparent;
  color: #FFFFFF;
}
