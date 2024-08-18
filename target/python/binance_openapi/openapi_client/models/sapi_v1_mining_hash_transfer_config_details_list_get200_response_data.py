# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictInt
from typing import Any, ClassVar, Dict, List
from openapi_client.models.sapi_v1_mining_hash_transfer_config_details_list_get200_response_data_config_details_inner import SapiV1MiningHashTransferConfigDetailsListGet200ResponseDataConfigDetailsInner
from typing import Optional, Set
from typing_extensions import Self

class SapiV1MiningHashTransferConfigDetailsListGet200ResponseData(BaseModel):
    """
    SapiV1MiningHashTransferConfigDetailsListGet200ResponseData
    """ # noqa: E501
    config_details: List[SapiV1MiningHashTransferConfigDetailsListGet200ResponseDataConfigDetailsInner] = Field(alias="configDetails")
    total_num: StrictInt = Field(alias="totalNum")
    page_size: StrictInt = Field(alias="pageSize")
    __properties: ClassVar[List[str]] = ["configDetails", "totalNum", "pageSize"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of SapiV1MiningHashTransferConfigDetailsListGet200ResponseData from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of each item in config_details (list)
        _items = []
        if self.config_details:
            for _item in self.config_details:
                if _item:
                    _items.append(_item.to_dict())
            _dict['configDetails'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SapiV1MiningHashTransferConfigDetailsListGet200ResponseData from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "configDetails": [SapiV1MiningHashTransferConfigDetailsListGet200ResponseDataConfigDetailsInner.from_dict(_item) for _item in obj["configDetails"]] if obj.get("configDetails") is not None else None,
            "totalNum": obj.get("totalNum"),
            "pageSize": obj.get("pageSize")
        })
        return _obj


