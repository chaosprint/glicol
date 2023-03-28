use pest::Parser;
use pest::error::Error;
use pest_derive::*;
// use pest::iterators::Pair;
// use pest::error::ErrorVariant;
use hashbrown::HashMap;

use glicol_macros::{one_para_number_or_ref, two_numbers};
use glicol_synth::GlicolPara;
use fasteval;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

pub fn get_num(para: GlicolPara) -> f32 {
    match para {
        GlicolPara::Number(v) => v,
        _ => 0.0
    }
}

/// index, (vector of chain name, vector of parameter list)
pub type GlicolAst = HashMap<String, (Vec<String>, Vec<Vec<GlicolPara>>)>;

pub fn get_ast(code: &str) -> Result<GlicolAst, Error<Rule>> {
    let mut block = match GlicolParser::parse(Rule::block, code) {
        Ok(v) => v,
        Err(e) => { return Err(e) }
    };
    // this can be a comment though, but we call it a line
    let lines = block.next().unwrap();
    let mut ast = GlicolAst::new();
    for line in lines.into_inner() {
        match line.as_rule() {
            Rule::line => {
                // println!("each line {:?}", line.as_str());
                let mut key = "";
                let mut chain_node_names = vec![];
                let mut chain_paras = vec![];
                for line_component in line.into_inner() {
                    match line_component.as_rule() {
                        Rule::reference => {
                            // println!("ref {:?}", line_component.as_str());
                            key = line_component.as_str();
                        },
                        Rule::chain => {
                            // println!("chain {:?}", line_component.as_str());
                            let chain = line_component;
                            for node_pair in chain.into_inner() {
                                let node = node_pair.into_inner().next().unwrap();
                                match node.as_rule() {
                                    Rule::point_node => {
                                        // println!("node {:?}", node.as_str()); //"all params"
                                        chain_node_names.push("points");
                                        let mut vec = vec![];
                                        let mut span = -1.0;
                                        let mut is_looping = false;
                                        let mut node_inner = node.into_inner();
                                        let points = node_inner.next().unwrap(); // it should be valid

                                        if let Some(math_or_loop) = node_inner.next() {
                                            match math_or_loop.as_rule() {
                                                Rule::math_expression => {
                                                    let mut one = "1".to_owned();
                                                    let mut ns = fasteval::EmptyNamespace;
                                                    one.push_str(math_or_loop.as_str());
                                                    span = fasteval::ez_eval(&one, &mut ns).unwrap() as f32;
                                                    if node_inner.next().is_some() {
                                                        is_looping = true;
                                                    };
                                                },
                                                Rule::is_looping => {
                                                    span = 1.0;
                                                    is_looping = true;
                                                },
                                                _ => {}
                                            }
                                        }

                                        for point in points.into_inner() {
                                            // println!("point {:?} ", point.as_str());  
                                            let mut point_inner = point.into_inner();
                                            let time = point_inner.next().unwrap();
                                            let mut time_inner = time.into_inner();
                                            let bar = time_inner.next().unwrap();

                                            let mut time_list = match bar.as_rule() {
                                                Rule::number => 
                                                    vec![GlicolPara::Bar(bar.as_str().parse::<f32>().unwrap())],
                                                Rule::bar => {
                                                    let bar_div: Vec<f32> = bar.as_str().split("/").map(|x|{
                                                        x.parse::<f32>().unwrap()
                                                    }).collect();
                                                    let num = bar_div[0] / bar_div[1];
                                                    vec![GlicolPara::Bar(num)]
                                                },
                                                _ => unimplemented!()
                                            };
                                            let sign_rule = time_inner.next();
                                            if sign_rule.is_some() {
                                                let mut sign = 1.0;
                                                if sign_rule.unwrap().as_str() == "-" {
                                                    sign = -1.0;
                                                }

                                                let s = time_inner.next().unwrap();
                                                match s.as_rule() {
                                                    Rule::second => time_list.push(
                                                        GlicolPara::Second(
                                                            sign * s.as_str().replace("_s", "").parse::<f32>().unwrap())
                                                    ),
                                                    Rule::ms => time_list.push(
                                                        GlicolPara::Millisecond(
                                                            sign * s.as_str().replace("_ms", "").parse::<f32>().unwrap())
                                                    ),
                                                    _ => unimplemented!()
                                                
                                                };

                                            };
                                            
                                            let t = GlicolPara::Time(time_list);
                                            let value = point_inner.next().unwrap().as_str().parse::<f32>().unwrap();
                                            // println!("value {:?} ", value);
                                            vec.push((t, GlicolPara::Number(value) ));
                                        }
                                        
                                        chain_paras.push(vec![GlicolPara::Points(vec), GlicolPara::Number(span), GlicolPara::Bool(is_looping)]);
                                        
                                    }
                                    Rule::delayn => one_para_number_or_ref!("delayn"),
                                    Rule::delayms => one_para_number_or_ref!("delayms"),
                                    Rule::imp =>  one_para_number_or_ref!("imp"),
                                    Rule::tri =>  one_para_number_or_ref!("tri"),
                                    Rule::squ => one_para_number_or_ref!("squ"),
                                    Rule::saw => one_para_number_or_ref!("saw"),
                                    Rule::onepole => one_para_number_or_ref!("onepole"),
                                    Rule::sin => one_para_number_or_ref!("sin"),
                                    Rule::mul => one_para_number_or_ref!("mul"),
                                    Rule::add => one_para_number_or_ref!("add"),
                                    Rule::seq => {
                                        let mut event = Vec::<(f32, GlicolPara)>::new();
                                        // println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        // println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("seq");
                                        // to do, more than a symbol
                                        // should be an event that contains time and note
                                        // GlicolPara::Symbol(paras.as_str())
                                        let compounds: Vec<_> = paras.into_inner().collect();
                                        // one bar will firstly be divided here
                                        let compounds_num = compounds.len();
                                        // println!("{:?}", );
                                        for (i, compound) in compounds.into_iter().enumerate() {
                                            let relative_time_base = i as f32 /compounds_num as f32;
                                            let elements: Vec<_> = compound.into_inner().collect();
                                            let elements_n = elements.len();

                                            for (j, element) in elements.into_iter().enumerate() {
                                                let relative_time_sub = 1./ compounds_num as f32 * j as f32 / elements_n as f32;
                                                let e = element.into_inner().next().unwrap();
                                                let time = relative_time_sub + relative_time_base;
                                                match e.as_rule() {
                                                    Rule::integer => {
                                                        event.push( (time, GlicolPara::Number(e.as_str().parse::<f32>().unwrap()) ));
                                                        // println!("int {:?}", e.as_str());
                                                    },
                                                    Rule::rest => {
                                                        // println!("rest {:?}", e.as_str());
                                                        // event.push( (time , GlicolPara::Number(0.0) ));
                                                    },
                                                    Rule::note_ref => {
                                                        // println!("ref {:?}", e.as_str());
                                                        event.push( (time, GlicolPara::Reference(e.as_str().to_owned()) ));
                                                    },
                                                    _=> unimplemented!()
                                                }
                                            }
                                        }
                                        chain_paras.push(vec![GlicolPara::Sequence(event)]);
                                    },
                                    Rule::choose => {
                                        // println!("node {:?}", node.as_str());
                                        let paras: Vec<_> = node.into_inner().map(|x|x.as_str().parse::<f32>().unwrap()).collect();
                                        // println!("paras {:?}", paras);
                                        chain_node_names.push("choose");
                                        chain_paras.push(vec![GlicolPara::NumberList(paras)]);
                                    },
                                    Rule::mix => {
                                        // println!("node {:?}", node.as_str());
                                        let paras: Vec<_> = node.into_inner().map(|x| GlicolPara::Reference(x.as_str().to_owned()) ).collect();
                                        // println!("paras {:?}", paras);
                                        chain_node_names.push("mix");
                                        chain_paras.push(paras);
                                    },
                                    Rule::sp => {
                                        // println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        // println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("sp");
                                        chain_paras.push(vec![GlicolPara::SampleSymbol(paras.as_str().to_owned())]);
                                    },
                                    Rule::speed => one_para_number_or_ref!("speed"),
                                    Rule::constsig => {
                                        // println!("node {:?}", node.as_str()); //"sin 440"
                                        let paras = node.into_inner().next().unwrap();
                                        // println!("paras {:?}", paras.as_str());//"440"                                        
                                        chain_node_names.push("constsig");
                                        match paras.as_rule() {
                                            Rule::number => {
                                                chain_paras.push(vec![GlicolPara::Number(paras.as_str().parse::<f32>().unwrap())]);
                                            },
                                            Rule::reference => {
                                                chain_paras.push(vec![GlicolPara::Reference(paras.as_str().to_owned())]);
                                            },
                                            Rule::event => {
                                                let mut p1i = paras.into_inner();
                                                let p: Vec<(GlicolPara, f32)> = p1i.next().unwrap().into_inner()
                                                .map(|pair| {
                                                    let mut it = pair.as_str().split("@");
                                                    let value = GlicolPara::Number(
                                                        it.next().unwrap().parse::<f32>().unwrap());
                                                    let time = it.next().unwrap().parse::<f32>().unwrap();
                                                    (value, time)
                                                }).collect();
                                                chain_paras.push(vec![GlicolPara::Event(p)])
                                            }
                                            ,
                                            Rule::pattern => {
                                                let mut p1i = paras.into_inner();
                                                
                                                let p: Vec<(GlicolPara, f32)> = p1i.next().unwrap().into_inner()
                                                .map(|pair| {
                                                    let mut it = pair.as_str().split("@");
                                                    let value = GlicolPara::Number(
                                                        it.next().unwrap().parse::<f32>().unwrap());
                                                    let time = it.next().unwrap().parse::<f32>().unwrap();
                                                    (value, time)
                                                }).collect();
                                                // println!("{:?}", p1i.next().unwrap());
                                                let span = match p1i.next() {
                                                    Some(r) => r.as_str().parse::<f32>().unwrap(),
                                                    None => 1.0
                                                };
                                                chain_paras.push(vec![GlicolPara::Pattern(p, span)])
                                            },
                                            _ => {}
                                        }
                                    }
                                    Rule::adc => one_para_number_or_ref!("adc"),
                                    Rule::bd => one_para_number_or_ref!("bd"),
                                    Rule::sn => one_para_number_or_ref!("sn"),
                                    Rule::hh => one_para_number_or_ref!("hh"),
                                    Rule::sawsynth => two_numbers!("sawsynth"),
                                    Rule::squsynth => two_numbers!("squsynth"),
                                    Rule::trisynth => two_numbers!("trisynth"),
                                    Rule::lpf => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("lpf");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str().to_owned())
                                                ,
                                                Rule::event => {
                                                    let mut p1i = p1.into_inner();
                                                    
                                                    let p: Vec<(GlicolPara, f32)> = p1i.next().unwrap().into_inner()
                                                    .map(|pair| {
                                                        let mut it = pair.as_str().split("@");
                                                        let value = GlicolPara::Number(
                                                            it.next().unwrap().parse::<f32>().unwrap());
                                                        let time = it.next().unwrap().parse::<f32>().unwrap();
                                                        (value, time)
                                                    }).collect();
                                                    GlicolPara::Event(p)
                                                }
                                                ,
                                                Rule::pattern => {
                                                    let mut p1i = p1.into_inner();
                                                    
                                                    let p: Vec<(GlicolPara, f32)> = p1i.next().unwrap().into_inner()
                                                    .map(|pair| {
                                                        let mut it = pair.as_str().split("@");
                                                        let value = GlicolPara::Number(
                                                            it.next().unwrap().parse::<f32>().unwrap());
                                                        let time = it.next().unwrap().parse::<f32>().unwrap();
                                                        (value, time)
                                                    }).collect();
                                                    // println!("{:?}", p1i.next().unwrap());
                                                    let span = match p1i.next() {
                                                        Some(r) => r.as_str().parse::<f32>().unwrap(),
                                                        None => 1.0
                                                    };
                                                    GlicolPara::Pattern(p, span)
                                                }
                                                ,
                                                _ => unimplemented!()
                                            },
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::psampler => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        // let p2 = iter.next().unwrap();
                                        chain_node_names.push("psampler");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str().to_owned())
                                                ,
                                                Rule::event => {
                                                    let mut p1i = p1.into_inner();
                                                    let p: Vec<(GlicolPara, f32)> = p1i.next().unwrap().into_inner()
                                                    .map(|pair| {
                                                        let mut it = pair.as_str().split("@");
                                                        let value_raw = it.next().unwrap();
                                                        let value = match value_raw.parse::<f32>() {
                                                            Ok(v) => GlicolPara::Number(v),
                                                            Err(_) => GlicolPara::Symbol(value_raw.to_owned())
                                                        };
                                                        let time = it.next().unwrap().parse::<f32>().unwrap();
                                                        (value, time)
                                                    }).collect();
                                                    GlicolPara::Event(p)
                                                }
                                                ,
                                                Rule::pattern => {
                                                    let mut p1i = p1.into_inner();
                                                    let p: Vec<(GlicolPara, f32)> = p1i.next().unwrap().into_inner()
                                                    .map(|pair| {
                                                        let mut it = pair.as_str().split("@");
                                                        let value_raw = it.next().unwrap();
                                                        let value = match value_raw.parse::<f32>() {
                                                            Ok(v) => GlicolPara::Number(v),
                                                            Err(_) => GlicolPara::Symbol(value_raw.to_owned())
                                                        };
                                                        let time = it.next().unwrap().parse::<f32>().unwrap();
                                                        (value, time)
                                                    }).collect();
                                                    let span = match p1i.next() {
                                                        Some(r) => r.as_str().parse::<f32>().unwrap(),
                                                        None => 1.0
                                                    };
                                                    GlicolPara::Pattern(p, span)
                                                }
                                                ,
                                                _ => unimplemented!()
                                            },
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::balance => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("balance");
                                        chain_paras.push(vec![
                                            GlicolPara::Reference(p1.as_str().to_owned()),
                                            GlicolPara::Reference(p2.as_str().to_owned()),
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::rhpf => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("rhpf");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str().to_owned())
                                                ,
                                                _ => unimplemented!()
                                            },
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::apfmsgain => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("apfmsgain");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str().to_owned())
                                                ,
                                                _ => unimplemented!()
                                            },
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::reverb => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        let p3 = iter.next().unwrap();
                                        let p4 = iter.next().unwrap();
                                        let p5 = iter.next().unwrap();
                                        // let p6 = iter.next().unwrap();
                                        chain_node_names.push("reverb");
                                        chain_paras.push(vec![
                                            GlicolPara::Number(p1.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p3.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p4.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p5.as_str().parse::<f32>().unwrap()),
                                            // GlicolPara::Number(p6.as_str().parse::<f32>().unwrap()),
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::envperc => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("envperc");
                                        chain_paras.push(vec![
                                            GlicolPara::Number(p1.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
                                    },
                                    Rule::adsr => {
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        let p3 = iter.next().unwrap();
                                        let p4 = iter.next().unwrap();
                                        chain_node_names.push("adsr");
                                        chain_paras.push(vec![
                                            GlicolPara::Number(p1.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p3.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(p4.as_str().parse::<f32>().unwrap()),
                                        ]);
                                    },
                                    Rule::plate => one_para_number_or_ref!("plate"),
                                    Rule::get => one_para_number_or_ref!("get"),
                                    Rule::noise => one_para_number_or_ref!("noise"),
                                    Rule::meta => {
                                        // println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        // println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("meta");
                                        chain_paras.push(vec![GlicolPara::Symbol(paras.as_str().replace("`", "").to_owned())]);
                                    },
                                    Rule::expr => {
                                        // println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        // println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("expr");
                                        chain_paras.push(vec![GlicolPara::Symbol(paras.as_str().replace("`", "").to_owned())]);
                                    },
                                    Rule::eval => {
                                        // println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        // println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("eval");
                                        // we do some checkings here
                                        // parse first
                                        // then run later

                                        chain_paras.push(vec![GlicolPara::Symbol(paras.as_str().replace("`", "").to_owned())]);
                                    },
                                    Rule::arrange => {
                                        // println!("node {:?}", node.as_str());
                                        let paras: Vec<_> = node.into_inner().map(|x|
                                            match x.as_rule() {
                                                Rule::reference => GlicolPara::Reference(x.as_str().to_owned()),
                                                Rule::number => {
                                                    // TODO: report error if < 0, or do it in pest
                                                    GlicolPara::Number( x.as_str().parse::<f32>().unwrap())
                                                },
                                                _ => unimplemented!()
                                            }
                                        ).collect();
                                        // println!("paras {:?}", paras);
                                        chain_node_names.push("arrange");
                                        chain_paras.push(paras);
                                    },
                                    Rule::msgsynth => {
                                        chain_node_names.push("msgsynth");
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let mut paras = vec![];
                                        let p1 = iter.next().unwrap();
                                        paras.push(GlicolPara::Symbol(p1.as_str().to_owned()));
                                        let p2 = iter.next();
                                        if p2.is_some() {
                                            paras.push(GlicolPara::Number(p2.unwrap().as_str().parse::<f32>().unwrap()));
                                        }
                                        let p3 = iter.next();
                                        if p3.is_some() {
                                            paras.push(GlicolPara::Number(p3.unwrap().as_str().parse::<f32>().unwrap()));
                                        }
                                        chain_paras.push(paras)
                                    },
                                    Rule::pattern_synth => {
                                        chain_node_names.push("pattern_synth");
                                        // println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let mut paras = vec![];
                                        let p1 = iter.next().unwrap();
                                        paras.push(GlicolPara::Symbol(p1.as_str().to_owned()));
                                        let p2 = iter.next();
                                        if p2.is_some() {
                                            paras.push(GlicolPara::Number(p2.unwrap().as_str().parse::<f32>().unwrap()));
                                            
                                        }
                                        chain_paras.push(paras)
                                    },
                                    _ => unimplemented!()
                                }
                            }
                            // println!("chain.into_inner(); {:?}", chain.into_inner());
                        },
                        _ => {}
                    }
                }
                ast.insert(
                    key.to_owned(), 
                    (
                        chain_node_names.iter_mut().map(|x|x.to_owned()).collect::<Vec<String>>(), 
                        chain_paras
                    )
                );
            },
            _ => {},
        };
    }
    Ok(ast)
}