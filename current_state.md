## Observations:
- Body of static/index.html will be replaced immediately after the app is loaded
- Can embed button into table column
	<td><button onclick=|_| Msg::Indexes,>{ "IndexesPage" }</button></td>
- Can iterate through vector of structs like below
	for self.indexesPage.indexes.iter().map(|i| html! {
                <tr> <td> { i.id } </td> <td> { i.name.to_owned() } </td> <td><button onclick=|_| Msg::Indexes,>{ "IndexesPage" }</button></td></tr>
            })

