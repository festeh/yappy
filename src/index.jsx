import React from "react";
import ReactDOM from "react-dom/client";

import App from "./App";
import Settings from "./Settings";
import Layout from "./layout";
import Stats from "./Stats";
import Tasks from "./Tasks";
import { Switch, Route } from "wouter";


const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(
	<Layout>
		<Switch>
			<Route path="/" component={App} />
			<Route path="/stats" component={Stats} />
			<Route path="/tasks" component={Tasks} />
			<Route path="/settings" component={Settings} />
		</Switch>
	</Layout>
);
