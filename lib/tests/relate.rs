mod parse;
use parse::Parse;
mod helpers;
use helpers::new_ds;
use surrealdb::dbs::Session;
use surrealdb::err::Error;
use surrealdb::sql::Value;

#[tokio::test]
async fn relate_with_parameters() -> Result<(), Error> {
	let sql = "
		LET $tobie = person:tobie;
		LET $jaime = person:jaime;
		RELATE $tobie->knows->$jaime SET id = knows:test, brother = true;
	";
	let dbs = new_ds().await?;
	let ses = Session::owner().with_ns("test").with_db("test");
	let res = &mut dbs.execute(sql, &ses, None).await?;
	assert_eq!(res.len(), 3);
	//
	let tmp = res.remove(0).result?;
	let val = Value::None;
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::None;
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"[
			{
				id: knows:test,
				in: person:tobie,
				out: person:jaime,
				brother: true,
			}
		]",
	);
	assert_eq!(tmp, val);
	//
	Ok(())
}

#[tokio::test]
async fn relate_and_overwrite() -> Result<(), Error> {
	let sql = "
		LET $tobie = person:tobie;
		LET $jaime = person:jaime;
		RELATE $tobie->knows->$jaime CONTENT { id: knows:test, brother: true };
		UPDATE knows:test CONTENT { test: true };
		SELECT * FROM knows:test;
	";
	let dbs = new_ds().await?;
	let ses = Session::owner().with_ns("test").with_db("test");
	let res = &mut dbs.execute(sql, &ses, None).await?;
	assert_eq!(res.len(), 5);
	//
	let tmp = res.remove(0).result?;
	let val = Value::None;
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::None;
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"[
			{
				id: knows:test,
				in: person:tobie,
				out: person:jaime,
				brother: true,
			}
		]",
	);
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"[
			{
				id: knows:test,
				in: person:tobie,
				out: person:jaime,
				test: true,
			}
		]",
	);
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"[
			{
				id: knows:test,
				in: person:tobie,
				out: person:jaime,
				test: true,
			}
		]",
	);
	assert_eq!(tmp, val);
	//
	Ok(())
}

#[tokio::test]
async fn relate_only() -> Result<(), Error> {
	let sql: &str = "
		RELATE ONLY test:1->edge:a->test:2;
		RELATE ONLY []->edge:b->test:3;
		RELATE ONLY [test:4, test:5]->edge:c->test:6;
	";
	let dbs = new_ds().await?;
	let ses = Session::owner().with_ns("test").with_db("test");
	let res = &mut dbs.execute(sql, &ses, None).await?;
	assert_eq!(res.len(), 3);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse(
		"{
			id: edge:a,
			in: test:1,
			out: test:2
		}",
	);
	assert_eq!(tmp, val);
	//
	let tmp = res.remove(0).result?;
	let val = Value::parse("NONE");
	assert_eq!(tmp, val);
	//
	match res.remove(0).result {
		Err(surrealdb::error::Db::SingleOnlyOutput) => (),
		_ => panic!("Query should have failed with error: Expected a single result output when using the ONLY keyword")
	}
	//
	Ok(())
}
