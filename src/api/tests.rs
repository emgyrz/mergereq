use super::*;

fn get_gl<'a>() -> GLApi<'a> {
  let req_params = ReqParams {
    private_token: Some(""),
    repo_url: Some(""),
    default_project: None,
  };

  GLApi::init(req_params)
}

#[test]
fn projects() -> Result<(), Box<dyn std::error::Error>> {
  let gl = get_gl();
  let q = projects::GetProjectsQuery::new().archived(false);
  // .membership(true);

  let pr = gl.get_projects(&q);
  println!(">>>{:#?}", pr);
  Ok(())
}

#[test]
fn users() -> Result<(), Box<dyn std::error::Error>> {
  let gl = get_gl();

  let q = users::GetUsersQuery::new().state(users::UserState::Active);
  // .username("mz2");

  let pr = gl.get_users(&q);
  println!(">>>{:#?}", pr);
  Ok(())
}

#[test]
fn mr() -> Result<(), Box<dyn std::error::Error>> {
  let gl = get_gl();

  let q = merge_requests::GetMergeRequestsQuery::new().state(merge_requests::MRState::Opened);

  let pr = gl.get_project_merge_requests(Some("80"), &q)?;
  for p in &pr {
    println!("{:#?}", p);
  }
  println!(">>>{:#?}", pr.len());
  Ok(())
}

#[test]
fn branches() -> Result<(), Box<dyn std::error::Error>> {
  let gl = get_gl();

  let q = branches::GetBranchesQuery::new();

  let br = gl.get_project_branches(Some("80"), &q)?;
  for b in &br {
    println!("{:#?}", b);
  }
  println!(">>>{:#?}", br.len());
  Ok(())
}
